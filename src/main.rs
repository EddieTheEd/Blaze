use std::fs::{self, File, DirEntry};
use std::time::UNIX_EPOCH;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use time::macros::date;
use serde::Deserialize;
use toml::Value;
use markdown::{to_html_with_options, Constructs};
use regex::Regex;

#[derive(Deserialize, Debug)]
struct Config {
    build: Build,
    settings: Settings,
}

#[derive(Deserialize, Debug)]
struct Build {
    input: String,
    output: String,
    ignore: toml::value::Array,
    theme: String,
    verbose: Option<bool>,
    base_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Settings {
    graph: Option<bool>,
}

#[derive(Debug)]
enum FsThing {
    File { path: String, content: String, metadata: Metadata },
    Directory { path: String, contents: Vec<FsThing>, metadata: Metadata }
}

// wrapper function to format a date
fn format_date (time: u64) -> String {
    let mut date = date!(1970 - 1 - 1);
    let days = time / 86400;

    for _ in 0..days {
        date = date.next_day().expect("she'll be right");
    }

    let format = time::format_description::parse("[day]-[month]-[year]").expect("hardcoded");

    date.format(&format).unwrap()
}

#[derive(Debug)]
struct Metadata {
    created: u64,
    accessed: u64,
    modified: u64,
}

struct Frontmatter {
    title: Option<String>,
    description: Option<String>,
}

#[derive(Clone, Debug)]
struct LinkTarget {
    path: String,
    // title: String,
}

#[derive(Debug)]
struct PageInfo {
    title: String,
    description: Option<String>,
    created: u64,
    accessed: u64,
    modified: u64,
}

impl Frontmatter {
    fn new () -> Self {
        Frontmatter 
            { 
                title: None, 
                description: None
            }
    }
}

impl Metadata {
    fn new (md: std::fs::Metadata) -> Self {
        let created = match md.created().unwrap().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("what the fuck")
        };
        let accessed = match md.accessed().unwrap().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("what the fuck")
        };
        let modified = match md.modified().unwrap().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("what the fuck")
        };

        Metadata { created, accessed, modified }
    }
}

fn format_path (path: &String) -> String {
    // replace \ with /
    let path = path.replace("\\", "/");

    // // replace up to the last / with spaces
    // let mut path = path.split("/");

    // let mut formatted_path = "".to_string();
    // let mut last_part = "";

    // path.for_each(|part| {
    //     formatted_path.push_str(" ".repeat(last_part.len()).as_str());
    //     last_part = part;
    // });

    // formatted_path.push_str(last_part);

    // formatted_path

    path
}

fn walk_directory(path: &str, cfg: &Config, ignore: &str) -> Vec<FsThing> {
    let this_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(why) => {
            println!("error reading path {}: {}", path,why);
            return vec![]
        }
    };

    let entries: Vec<DirEntry> = this_dir
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap())
        .collect();

    entries.iter()
        .map(|entry| {

            println!("found {}", format_path(&entry.path().display().to_string()));
            let metadata = fs::metadata(entry.path()).unwrap();
            let metadata = Metadata::new(metadata);
            if cfg.build.verbose.unwrap_or(false) {
                println!("created {}", format_date(metadata.created));
                println!("last accessed {}", format_date(metadata.accessed));
                println!("last modified {}", format_date(metadata.modified));
            }

            if cfg.build.ignore.iter().any(|i| match i {
                Value::String(s) => s.to_string() == entry.path().to_str().unwrap().replace("\\","/"),
                _ => false
            }) {
                println!("ignored (private)");
                return None
            }

            if entry.path().is_dir() {
                Some(FsThing::Directory { 
                    path: entry.path().display().to_string().chars().skip(ignore.len() + 1).collect::<String>(), 
                    contents: walk_directory(entry.path().to_str().expect("should not fail"), cfg, ignore), 
                    metadata
                })
            } else {
                let content = match std::fs::read_to_string(entry.path()) {
                    Ok(content) => content,
                    Err(why)      => {println!("failed to read file: {}", why); "".to_string()}
                };
                Some(FsThing::File { 
                    path: entry.path().display().to_string().chars().skip(ignore.len() + 1).collect::<String>(), 
                    content, 
                    metadata
                })
            }

        })
        .filter(|entry| entry.is_some())
        .map(|entry| entry.unwrap())
        .collect::<Vec<FsThing>>()

}

fn compile_markdown (
    things: &Vec<FsThing>, cfg: &Config, theme: &String, 
    links: &HashMap<String, Vec<LinkTarget>>, 
    backlinks: &HashMap<String, Vec<LinkTarget>>, 
    infos: &HashMap<String, PageInfo>, 
    partials: &HashMap<String,String>, 
    first_iteration: bool) {
    if first_iteration {
        // delete directory and create a new one
        match fs::remove_dir_all(&cfg.build.output) {
            Ok(_) => (),
            Err(why) => println!("error deleting old output: {}", why), // this is actually also fine because there is no directory (update: oh god it is not fine oh well)
        }
        fs::create_dir(&cfg.build.output).expect("no permissions");
    }
    for thing in things {
        match thing {
            FsThing::File { path, content, metadata } => {
                if path.ends_with(".md") || path.ends_with(".markdown") {

                    let html_path = if path.ends_with(".md") {
                        path.replace(".md",".html")
                    } else {
                        path.replace(".markdown",".html")
                    };

                    let mut dir = cfg.build.output.clone();
                    dir.push('/');
                    dir.push_str(html_path.as_str());
                    
                    println!("processing {}",dir);
                    let mut file = match fs::File::create(&dir) {
                        Ok(f) => f,
                        Err(why) => panic!("error creating file {}: {}", &dir, why)
                    };

                    // extract the frontmatter
                    let mut frontmatter: Frontmatter = Frontmatter::new();
                    
                    //modifying backlink content to account for links with .md
                    //temporary solution, this replaces all instances of .md, not
                    //[something](somethingelse.md)
                    let mut content = content.clone().replace(".md","");
                    let old_content = &content.clone().replace(".md","");

                    let crlfregex = Regex::new(r#"([^\r])\n"#).unwrap();

                    for cap in crlfregex.captures_iter(&old_content) {
                        let mut new = cap.get(1).unwrap().as_str().to_string();
                        new.push_str("\r\n");
                        content = content.replace(cap.get(0).unwrap().as_str(), &new);
                    }

                    let content = 
                        if content.starts_with("---") {
                            // frontmatter
                            let mut c_iter = content.split("\n");

                            c_iter.next();

                            loop {
                                match c_iter.next().unwrap_or("---").trim() {
                                    "---" => break,
                                    line => {
                                        let mut piter = line.split(":");
                                        let key = piter.next().unwrap();
                                        let val = piter.next().unwrap_or("");

                                        match key {
                                            "title" => frontmatter.title = Some(val.trim().to_string()),
                                            "description" => frontmatter.description = Some(val.trim().to_string()),
                                            _ => (),
                                        }
                                    }
                                        
                                }
                            };

                            c_iter.collect::<Vec<&str>>().join("\n")
                        } else {
                            content.clone()
                        };

                    let compiled_markdown = to_html_with_options(&content,
                        &markdown::Options { 
                            parse: markdown::ParseOptions { 
                                    constructs: Constructs {
                                        // frontmatter: true,
                                        math_flow: true,
                                        gfm_strikethrough: true,
                                        gfm_table: true,
                                        gfm_footnote_definition: true,
                                        gfm_label_start_footnote: true,
                                        ..Constructs::default()
                                    }, 
                                    ..Default::default() 
                                }, 
                                ..Default::default() 
                            }
                    ).unwrap();
                    
                    
                    let mut compiled_html = theme.clone();

                    let mut backlinks_partials = "".to_string();

                    if cfg.settings.graph.unwrap_or(false) {
                        let canonical_path = path.replace("\\", "/");
                        let canonical_path = if canonical_path.contains(".md") {
                            canonical_path.replace(".md", "")
                        } else {
                            canonical_path.replace(".markdown", "")
                        };

                        match backlinks.get(&canonical_path) {
                            Some(backlinks) => {
                                for link in backlinks {
                                    println!("--- backlink --- {}",link.path);
                                    match infos.get(&link.path) {
                                        Some(info) => {
                                            backlinks_partials.push_str(
                                                format!(r#"{{{{ partial "backlink.html" . ({}, {}, {}, {}) }}}}
"#, 
                                                    info.title,
                                                    link.path,
                                                    info.description.clone().unwrap_or("".to_string()),
                                                    info.modified).as_str()
                                            )
                                        }
                                        None => () // odd but fine
                                    }
                                }
                            },
                            None => () // odd but fine
                        }
                        println!("{}", backlinks_partials);

                        compiled_html = compiled_html.replace("{{backlinks}}", &backlinks_partials);
                    }

                    let regex = Regex::new(r#"\{\{\s*partial\s*"(.*)"\s*.\s*(?:\((.*)*\))?\s*\s*\}\}"#).unwrap();

                    loop {
                        match regex.captures(&compiled_html) {
                            Some(caps) => {
                                let partial = match partials.get(caps.get(1).unwrap().as_str()) {
                                    Some(p) => p,
                                    None => "",
                                };
                                let partial = if caps.get(1).unwrap().as_str() == "backlinks.html" {
                                    &backlinks_partials
                                } else {
                                    partial
                                };
                                let mut subs = vec![];
                                match caps.get(2) {
                                    Some(values) => {
                                        for val in values.as_str().split(",") {
                                            subs.push(val.to_string());
                                        }
                                    },
                                    None => (),
                                }
                                compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), partial);
                                for (i, sub) in subs.iter().enumerate() {
                                    compiled_html = compiled_html.replace(&format!("{{{{{}}}}}", i).to_string(), sub);
                                }
                            }
                            None => break
                        }
                    }

                    compiled_html = compiled_html.replace("{{content}}", &compiled_markdown);

                    match &cfg.build.base_url {
                        Some(url) => compiled_html = compiled_html.replace("{{base_url}}", url),
                        None => {
                            // get number of directories deep we are
                            let deepness = path.replace("\\","/").split("/").skip(1).count();
                            let mut relative_base = "".to_string();
                            for _ in 0..deepness {
                                relative_base.push_str("../");
                            };
                            compiled_html = compiled_html.replace("{{base_url}}", &relative_base)
                        },
                    };

                    match &frontmatter.title {
                        Some(title) => compiled_html = compiled_html.replace("{{title}}", title),
                        None => {
                            let last = html_path.replace("\\","/");
                            let last = last.split("/").last().expect("should have a path");
                            compiled_html = compiled_html.replace("{{title}}", last)
                        },
                    }
                    
                    compiled_html = compiled_html.replace("{{description}}", 
                        &frontmatter.description.unwrap_or("".to_string()));

                    compiled_html = compiled_html.replace("{{date_created}}", &format_date(metadata.created));
                    compiled_html = compiled_html.replace("{{last_modified}}", &format_date(metadata.created));
                    compiled_html = compiled_html.replace("{{last_accesssed}}", &format_date(metadata.created));

                    let callout_regex = Regex::new(r#"<blockquote>\s*<p>\s?\[!(\w+)]\s*([^<]*)\s*((?s).*)\s*</blockquote>"#).unwrap();
                    
                    loop {
                        match callout_regex.captures(&compiled_html) {
                            Some(caps) => {
                                let replacement = 
                                    // ossac can you make it so that the callout title has the callout type, i.e. "todo-title", literally same as the class
                                    // todo: make the theme dependent on the theme selected
                                    // the blaze.png is temporary
                                    format!("<blockquote id=\"callout\" class=\"{}-callout\"><div id='callout-header'><div id='{}' class='calloutimage'></div><h3 id='callout-title'>{}</h3></div><p>{}</p></blockquote>", 
                                        caps.get(1).unwrap().as_str(), 
                                        caps.get(1).unwrap().as_str(),
                                        caps.get(2).unwrap().as_str(),
                                        caps.get(3).unwrap().as_str(),);
                                // println!("{}", replacement);
                                compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), &replacement);
                            }
                            None => break
                        }
                    }

                    let link_regex = Regex::new(r#"<a href="((?:../)*)([^."]*)">((?s)[^<]*)</a>"#).unwrap();
                    let dotdotslash_regex = Regex::new(r#"([^/]*)/\.\./"#).unwrap();

                    loop {
                        match link_regex.captures(&compiled_html) {
                            Some(caps) => {
                                // let mut full_path = path.clone().replace("\\","/");
                                let mut full_path = "".to_string();
                                // full_path.push_str("/../");
                                full_path.push_str(caps.get(1).unwrap().as_str());
                                full_path.push_str(caps.get(2).unwrap().as_str());
                                if caps.get(2).unwrap().as_str().starts_with("/") {
                                    full_path = caps.get(2).unwrap().as_str().chars().skip(1).collect();
                                }
                                loop {
                                    match dotdotslash_regex.find(&full_path) {
                                        Some(mat) => 
                                            full_path = full_path.replace(mat.as_str(), ""),
                                        None => break
                                    }
                                }
                                full_path.push_str(".html"); // it brokey without this thats why there's so many debug
                                let replacement =
                                    format!("<a href=\"{}\">{}</a>",
                                        full_path,
                                        caps.get(3).unwrap().as_str());
                                // println!("{}", compiled_html);
                                compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), &replacement);
                                // println!("{}", compiled_html);
                                // break
                                
                            }
                            None => break
                        }
                    }

                    let latex_block_regex = Regex::new(r#"<pre><code class="language-math math-display">((?s)[^<]*)</code></pre>"#).unwrap();

                    loop {
                        match latex_block_regex.captures(&compiled_html) {
                            Some(caps) => {
                                let replacement = format!("$${}$$", caps.get(1).unwrap().as_str());
                                // println!("{}", &replacement);
                                compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), &replacement);
                            }
                            None => break
                        }
                    }

                    let latex_regex = Regex::new(r#"<code class="language-math math-display">((?s)[^<]*)</code>"#).unwrap();

                    loop {
                        match latex_regex.captures(&compiled_html) {
                            Some(caps) => {
                                let replacement = format!("${}$", caps.get(1).unwrap().as_str());
                                // println!("{}", &replacement);
                                compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), &replacement);
                            }
                            None => break
                        }
                    }

                    // highlighting. turns out this breaks a ton of shit!!

                    // let highlight_regex = Regex::new(r#"==((?s).*)=="#).unwrap();

                    // loop {
                    //     match highlight_regex.captures(&compiled_html) {
                    //         Some(caps) => {
                    //             let replacement = format!("<mark>{}</mark>", caps.get(1).unwrap().as_str());
                    //             println!("{}", &replacement);
                    //             compiled_html = compiled_html.replace(caps.get(0).unwrap().as_str(), &replacement);
                    //         }
                    //         None => break
                    //     }
                    // }
                    

                    match file.write(compiled_html.as_bytes()) {
                        Ok(_) => (),
                        Err(why) => panic!("error writing {}: {}", dir, why)
                    }
                } else {
                    // copy file directly
                    let mut in_dir = cfg.build.input.clone();
                    in_dir.push('/');
                    in_dir.push_str(path.as_str());
                    let mut out_dir = cfg.build.output.clone();
                    out_dir.push('/');
                    out_dir.push_str(path.as_str());
                    println!("copying file {} -> {}", path, out_dir);
                    let _ = fs::copy(in_dir, out_dir);
                }
            }
            FsThing::Directory { path, contents, metadata: _ } => {
                let mut dir = cfg.build.output.clone();
                dir.push('/');
                dir.push_str(path.as_str());
                fs::create_dir_all(dir).expect("error creating directory");

                compile_markdown(contents, cfg, theme, links, backlinks, infos, partials, false)
            }
        }
    }
}

fn copy_theme_files(things: &Vec<FsThing>, cfg: &Config, theme_path: &str) {
    for thing in things {
        match thing {
            FsThing::File { path, content: _, metadata: _ } => {
                if path.ends_with(".html") {
                    // skip that
                } else {
                    // copy file directly
                    let mut in_dir = theme_path.to_string();
                    in_dir.push('/');
                    in_dir.push_str(path.as_str());
                    let mut out_dir = cfg.build.output.clone();
                    out_dir.push('/');
                    out_dir.push_str(path.as_str());
                    println!("copying file {} -> {}", path, out_dir);
                    let _ = fs::copy(in_dir, out_dir);
                }
            }
            FsThing::Directory { path, contents, metadata: _ } => {
                if path == "partials" { continue }
                let mut dir = cfg.build.output.clone();
                dir.push('/');
                // println!("{}", path);
                dir.push_str(path.as_str());
                fs::create_dir_all(dir).expect("error creating directory");

                copy_theme_files(contents, cfg, theme_path);
            }
        }
    }
}

fn generate_backlinks<'a> (things: &Vec<FsThing>, cfg: &Config,
    links: &mut HashMap<String, Vec<LinkTarget>>, 
    backlinks: &mut HashMap<String, Vec<LinkTarget>>,
    infos: &mut HashMap<String, PageInfo>) {
        for thing in things {
            match thing {
                FsThing::File { path, content, metadata } => {
                    //modifying backlink content to account for links with .md
                    //temporary solution, this replaces all instances of .md, not
                    //[something](somethingelse.md)
                    let content = &content.clone().replace(".md","");
                    let link_regex = Regex::new(r"[^!]\[([^\[\]]*)\]\(([^\(\)]*)\)").unwrap();

                    let path = if path.ends_with(".md") {
                        path.replace(".md",".html")
                    } else {
                        path.replace(".markdown",".html")
                    };

                    for cap in link_regex.captures_iter(content) {
                        let dotdotslash_regex = Regex::new(r#"([^/]*)/\.\./"#).unwrap();

                        let mut linked_path = path.replace("\\","/");
                        linked_path.push_str("/../");
                        linked_path.push_str(cap.get(2).unwrap().as_str());
                        loop {
                            match dotdotslash_regex.find(&linked_path) {
                                Some(mat) => 
                                    linked_path = linked_path.replace(mat.as_str(), ""),
                                None => break
                            }
                        }

                        if linked_path == path.replace("\\","/") { continue }
                        if !links.contains_key(&path.replace("\\","/")) {
                            links.insert(
                                path.clone().replace("\\","/"),
                                Vec::new()
                            );
                        }
                        let outlinks = links.get(&path.replace("\\","/")).unwrap();
                        let mut outlinks = outlinks.clone();
                        outlinks.push(
                            LinkTarget { 
                                path: linked_path.clone()
                            }
                        );
                        links.insert(
                            path.clone().replace("\\","/"),
                            outlinks
                        );
                        if !backlinks.contains_key(&linked_path) {
                            backlinks.insert(
                                linked_path.clone(),
                                Vec::new()
                            );
                        }
                        let blinks = backlinks.get(&linked_path).unwrap();
                        let mut blinks = blinks.clone();
                        blinks.push(
                            LinkTarget { 
                                path: path.replace("\\","/")
                            }
                        );
                        backlinks.insert(
                            linked_path,
                            blinks
                        );

                        let mut frontmatter = 
                            if content.starts_with("---") {
                                let mut frontmatter = Frontmatter { title: None, description: None };
                                // frontmatter
                                let mut c_iter = content.split("\n");

                                c_iter.next();

                                loop {
                                    match c_iter.next().unwrap_or("---").trim() {
                                        "---" => break,
                                        line => {
                                            let mut piter = line.split(":");
                                            let key = piter.next().unwrap();
                                            let val = piter.next().unwrap_or("");

                                            match key {
                                                "title" => frontmatter.title = Some(val.trim().to_string()),
                                                "description" => frontmatter.description = Some(val.trim().to_string()),
                                                _ => (),
                                            }
                                        }
                                            
                                    }
                                };

                                frontmatter
                            } else {
                                Frontmatter { title: None, description: None }
                            };

                        if frontmatter.title.is_none() {
                            let last = path.replace("\\","/");
                            let last = last.split("/").last().expect("should have a path");
                            frontmatter.title = Some(last.to_string());
                        }

                        infos.insert(
                            path.replace("\\", "/"),
                            PageInfo { 
                                title: frontmatter.title.unwrap(), 
                                description: frontmatter.description, 
                                created: metadata.created, 
                                accessed: metadata.accessed, 
                                modified: metadata.modified
                            }
                        );
                        
                    }
                }
                FsThing::Directory { path: _, contents, metadata: _ } => {
                    generate_backlinks(contents, cfg, links, backlinks, infos);
                }
            }
        }
    }

fn main() {
    // open blaze-config.toml
    let config_path = Path::new("blazeconfig.toml");
    let display = config_path.display();

    let mut config_file = match File::open(&config_path) {
        Err(why) => panic!("couldn't open config file ({}): {}", display, why),
        Ok(file)  => file,
    };

    let mut config_file_contents = String::new();
    match config_file.read_to_string(&mut config_file_contents) {
        Err(why) => panic!("couldn't read config file ({}): {}", display, why),
        Ok(_)           => ()
    };

    let config: Config = toml::from_str(&config_file_contents).unwrap();

    // println!("{:#?}", config);

    let mut theme_path = "blaze/themes/".to_string();
    theme_path.push_str(&config.build.theme);

    // check that folder exists
    match fs::read_dir(&theme_path) {
        Ok(_) => (),
        Err(why) => panic!("error reading theme {}: {}", config.build.theme, why)
    }

    let mut theme_html = theme_path.clone();
    theme_html.push_str("/_theme.html");

    let theme_html = match fs::read(theme_html) {
        Ok(f) => f,
        Err(why) => panic!("error reading theme {}: {}", config.build.theme, why)
    };

    let theme_html = String::from_utf8(theme_html).unwrap();

    let all_content = walk_directory(&config.build.input, &config, &config.build.input);

    // println!("{:#?}", all_content)

    let mut partials_path = theme_path.clone();
    partials_path.push_str("/partials");

    let partials: HashMap<String,String> = match fs::read_dir(partials_path) {
        Ok(entries) => {
            let entries: Vec<DirEntry> = entries.filter(|elem| elem.is_ok())
                .map(|elem| elem.unwrap())
                .collect();

            let mut partials: HashMap<String,String> = HashMap::new();

            for entry in entries {
                let content = match std::fs::read_to_string(entry.path()) {
                    Ok(content) => content,
                    Err(why)      => {
                        println!("failed to read file: {}", why); "".to_string();
                        continue
                    }
                };
                let compact_path = entry.path().to_str().unwrap().split("partials").last().unwrap().to_string().chars().skip(1).collect::<String>();
                println!("{}",compact_path);
                partials.insert(
                    compact_path.to_string(),
                    content
                );
            }

            partials
        },
        Err(why) => panic!("error reading partials folder: {}", why)
    };

    let mut links: HashMap<String, Vec<LinkTarget>> = HashMap::new();
    let mut backlinks: HashMap<String, Vec<LinkTarget>> = HashMap::new();
    let mut infos: HashMap<String, PageInfo> = HashMap::new();

    if config.settings.graph.unwrap_or(false) {
        generate_backlinks(&all_content, &config, &mut links, &mut backlinks, &mut infos);

        // println!("{:#?}", infos);
    }
    
    // now compile .md or .markdown files
    compile_markdown(&all_content, &config, &theme_html, &links, &backlinks, &infos, &partials, true);

    let theme_files = walk_directory(&theme_path, &config, &theme_path);

    copy_theme_files(&theme_files, &config, &theme_path);
}
