use std::fs::{self, File, DirEntry};
use std::time::UNIX_EPOCH;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::io::Write;
use time::macros::date;
use serde::Deserialize;
use toml::Value;
use markdown::{to_html_with_options, Constructs};
use regex::Regex;
use toml;
use std::fs::OpenOptions;
use std::process::Command;
use std::str;
use chrono::{NaiveDateTime, DateTime, Local, TimeZone};

#[derive(Deserialize, Debug)]
struct Config {
    settings: Settings,
    drafts: Drafts,
    deployment: Deployment,
    development: Development,
    build: Build,
}

#[derive(Deserialize, Debug)]
struct Settings {
    graph: Option<bool>,
    backlinks: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Drafts {
    use_file_description: Option<bool>,
    custom_draft_description: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct Deployment {
    vercelcleanurl: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Development {
    liveserverconfig: Option<bool>,
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

#[derive(Debug)]
enum FsThing {
    File { path: String, content: String, metadata: Metadata },
    Directory { path: String, contents: Vec<FsThing>, metadata: Metadata }
}

// static mut drafts = Vec::<T>::new();


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

#[derive(Debug)]
struct Frontmatter {
    title: Option<String>,
    description: Option<String>,
    draft: Option<String>,
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
                description: None,
                draft: None,
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

        Metadata { created,  accessed, modified }
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

fn convert_links(input: &str) -> String {
    let link_regex = Regex::new(r#"\[\[([^|\]]+)(?:\|([^]]+))?\]\]"#).unwrap();

    link_regex.replace_all(input, |caps: &regex::Captures| {
        if let Some(url) = caps.get(1) {
            if let Some(text) = caps.get(2) {
                // Format: [[URL|text]] - convert to [text](URL)
                format!("[{}]({})", text.as_str(), url.as_str().replace(" ", "%20"))
            } else {
                // Format: [[URL]] - convert to [URL](URL)
                format!("[{}]({})", url.as_str(), url.as_str().replace(" ", "%20"))
            }
        } else {
            // No match, return the original substring
            caps.get(0).unwrap().as_str().to_string()
        }
    }).to_string()
}

fn compile_markdown (
    things: &Vec<FsThing>, cfg: &Config, theme: &String, 
    links: &HashMap<String, Vec<LinkTarget>>, 
    backlinks: &HashMap<String, Vec<LinkTarget>>, 
    infos: &HashMap<String, PageInfo>, 
    partials: &HashMap<String,String>, 
    first_iteration: bool
    ) {
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
                    
                    // println!("processing {}",dir);
                    let mut file = match fs::File::create(&dir) {
                        Ok(f) => f,
                        Err(why) => panic!("error creating file {}: {}", &dir, why)
                    };

                    // extract the frontmatter
                    let mut frontmatter: Frontmatter = Frontmatter::new();
                    
                    //modifying backlink content to account for links with .md
                    //temporary solution, this replaces all instances of .md, not
                    //[something](somethingelse.md)
                    let mut content = content.clone().replace(".md)",")");
                    content = convert_links(&content); // converts wikilinks to mdlinks

                    let old_content = &content.clone().replace(".md)",")");

                    let mut pathways: HashMap<String, String> = HashMap::new();

                    let crlfregex = Regex::new(r#"([^\r])\n"#).unwrap();

                    for cap in crlfregex.captures_iter(&old_content) {
                        let mut new = cap.get(1).unwrap().as_str().to_string();
                        new.push_str("\r\n");
                        content = content.replace(cap.get(0).unwrap().as_str(), &new);
                    }

                    let mut content = 
                        if content.starts_with("---") {
                            // frontmatter
                            let mut c_iter = content.split("\n");

                            c_iter.next();

                            loop {
                                match c_iter.next().unwrap_or("---").trim() {
                                    "---" => break,
                                    "" => {
                                        //println!("space detected in frontmatter");
                                    },
                                    line => {
                                        let mut piter = line.split(":");
                                        let key = piter.next().unwrap();
                                        let val = piter.next().unwrap_or("");
                                        // dunno what unwrap_or does, so im just gonna replace lol

                                        match key {
                                            "title" => {
                                                // It unfortunately removes all quotation marks at the beginning and end of the title (First ', then ")
                                                frontmatter.title = Some(val.trim().trim_matches('\'').trim_matches('\"').to_string());
                                            },
                                            "description" => {
                                                frontmatter.description = Some(val.trim().trim_matches('\'').trim_matches('\"').to_string());
                                            },
                                            
                                            "draft" => {
                                                frontmatter.draft = Some(val.trim().trim_matches('\'').trim_matches('\"').to_string());
                                                if cfg.drafts.use_file_description.unwrap_or(true) {
                                                } else {
                                                    frontmatter.description = Some(cfg.drafts.custom_draft_description.clone());
                                                }
                                                
                                            },
                                            "theme" => {
                                                // Ignore for now
                                            }
                                            _ => (),
                                        }
                                    }
                                        
                                }
                            };

                            c_iter.collect::<Vec<&str>>().join("\n")
                        } else {
                            content.clone()
                    };
                    
                    content = content.replace("$$\\begin{align}", "$$\n\\begin{align}").replace("\\end{align}$$", "\\end{align}\n$$");
                    // cause obsidian and/or obsidian's latex suite are acting up. This is a
                    // specific enough replace that I'm confident it will cause any erroneous
                    // errors.

                    let compiled_markdown: String;  // Declare the variable outside the match

                    match &frontmatter.draft {
                        Some(draft) => {
                            if draft == "true" {
                                // for the future, this message should be read from the
                                // blazeconfig.toml file.
                                // DONE - 29/08/23 - Reaper
                                compiled_markdown = String::from(format!("<strong>{}</strong>", cfg.drafts.message));
                            } else {
                                compiled_markdown = to_html_with_options(
                                    &content,
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
                                    },
                                )
                                .unwrap();
                            }
                        }
                        None => {
                            compiled_markdown = to_html_with_options(
                                &content,
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
                                },
                            )
                            .unwrap();
                        }
                    }      

                    let mut compiled_html = theme.clone();

                    let mut backlinks_partials = "".to_string();

                    if cfg.settings.backlinks.unwrap_or(false) {
                        let canonical_path = path.replace("\\", "/").replace(" ","%20");
                        let canonical_path = if canonical_path.contains(".md") {
                            canonical_path.replace(".md", "")
                        } else {
                            canonical_path.replace(".markdown", "")
                        };

                        //println!("{}",&canonical_path);

                        match backlinks.get(&canonical_path) {
                            Some(backlinks) => {
                                
                                for link in backlinks {
                                    //println!("--- backlink --- {} from {}",link.path, html_path);
                                    let tempkey = "backlink".to_owned()+&link.path.to_string().replace(".html", "");
                                    let tempvalue = "/".to_owned() + &link.path.to_string().replace("output", "");
                                    pathways.insert(tempkey.to_string(), tempvalue.to_string());
                                    match infos.get(&link.path) {
                                        Some(info) => {
                                            backlinks_partials.push_str(
                                                format!(r#"{{{{ partial "backlink.html" . ({}, {}, {}, {}) }}}}
"#, 
                                                    info.title.trim_matches('\'').trim_matches('\"').to_string(),
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
                        //println!("{}", backlinks_partials);

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
                                        caps.get(1).unwrap().as_str().to_lowercase(), 
                                        caps.get(1).unwrap().as_str().to_lowercase(),
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
                                //println!("here is a link maybe: {}", full_path);
                                let rootpath = "/".to_owned()+&full_path;
                                pathways.insert("forwardlink".to_owned()+&full_path.to_string().replace("%20", " "), rootpath.to_string().replace("%20", " "));


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

                    compiled_html = compiled_html.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"");

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
                    //
                    if cfg.settings.graph.unwrap_or(false) {
                        // graphing time (only backlinks cause i dunno how to do forward links)
                        // probably would've been better to save as not a hashmap but oh well
                        let mut graphdata = String::from("{\"nodes\":[");

                        let root = &dir.replace("output/","").replace(".html", "");
                        let rootlink = &dir.replace("output/", "/");

                        graphdata.push_str(&format!("{{\"id\":\"{}\",\"link\":\"{}\",\"linktype\":\"var(--root)\"}}", root, rootlink));

                        for (key, value) in pathways.iter() {

                            if key.contains("backlink") {
                                let temp = format!(",{{\"id\":\"{}\",\"link\":\"{}\",\"linktype\":\"var(--blnode)\"}}", key, value.replace(".html", "")+".html").replace("backlink", "");
                                graphdata.push_str(&temp);
                            }
                            if key.contains("forwardlink"){
                                let temp = format!(",{{\"id\":\"{}\",\"link\":\"{}\",\"linktype\":\"var(--flnode)\"}}", key, value.replace(".html", "")+".html").replace("forwardlink", "");
                                graphdata.push_str(&temp);
                            }
                        }

                        // forward logic goes here
                        
                        graphdata.push_str("], \"links\":[");

                        let mut counter = 0;
                        for (key, _value) in pathways.iter() {
                            let temp = format!("{{\"source\":\"{}\",\"target\":\"{}\",\"value\":2}},", root, key).replace("backlink", "").replace("forwardlink", "");
                            //println!("Link: {} -> Value: {}", key, value);
                            graphdata.push_str(&temp);
                            counter = counter + 1;
                            
                        }
                        // forward logic goes here

                        if counter != 0 {
                            graphdata.pop();
                        }

                        graphdata.push_str("]}");

                        let graphfilename = format!("output/{}.json", root);
                        let globalfilename = format!("output/global.json");

                        let mut graphfile = File::create(graphfilename.clone()).expect("Failed to create file");

                        graphfile.write_all(graphdata.as_bytes()).expect("Failed to write to file");

                        let mut globalfile = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(globalfilename.clone())
                            .expect("Failed to open or create global file");
                        
                        globalfile.write_all(graphdata.as_bytes()).expect("Failed to write to file");

                        // println!("Data outputted to {}", graphfilename);
                    }
                    


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
                    // println!("copying file {} -> {}", path, out_dir);
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
                    if path.contains("README.md"){
                        //println!("readme file detected");
                        // also skip that
                    }
                    else {
                        // copy file directly
                        let mut in_dir = theme_path.to_string();
                        in_dir.push('/');
                        in_dir.push_str(path.as_str());
                        let mut out_dir = cfg.build.output.clone();
                        out_dir.push('/');
                        out_dir.push_str(path.as_str());
                        // println!("copying file {} -> {}", path, out_dir);
                        let _ = fs::copy(in_dir, out_dir);

                    }
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

fn walk_directory(path: &str, cfg: &Config, ignore: &str) -> Vec<FsThing> {
    let this_dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_why) => {
            // println!("error reading path {}: {}", path,why);
            return vec![]
        }
    };

    let entries: Vec<DirEntry> = this_dir
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap())
        .collect();

    entries.iter()
        .map(|entry| {

            // println!("found {}", format_path(&entry.path().display().to_string()));
            let metadata = fs::metadata(entry.path()).unwrap();
            let metadata = Metadata::new(metadata);
            if cfg.build.verbose.unwrap_or(false) {
                // println!("created {}", format_date(metadata.created));
                // println!("last accessed {}", format_date(metadata.accessed));
                // println!("last modified {}", format_date(metadata.modified));
            }

            if cfg.build.ignore.iter().any(|i| match i {
                Value::String(s) => s.to_string() == entry.path().to_str().unwrap().replace("\\","/"),
                _ => false
            }) {
                // println!("ignored (private)");
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
                    Err(_)    => {"hi".to_string()} // this may have caused issues
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
                    let content = &convert_links(&content.clone().replace(".md)",")"));
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
                                let mut frontmatter = Frontmatter { title: None, description: None, draft: None };
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
                                Frontmatter { title: None, description: None, draft: None }
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

fn lastmod(folder_path: &str, input: &str) -> String {
    let mut outputstring = String::new();
    let entries = fs::read_dir(folder_path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let output = Command::new("git")
                .arg("log")
                .arg("-1")
                .arg("--pretty=format:%ci")
                .arg(path.clone())
                .output()
                .expect("Failed to execute command");


            let output_str = str::from_utf8(&output.stdout).expect("Output not UTF-8");
            let naive_date_time = NaiveDateTime::parse_from_str(output_str.trim(), "%Y-%m-%d %H:%M:%S %z")
                .expect("Failed to parse date");

            let local_date_time: DateTime<Local> = Local.from_local_datetime(&naive_date_time).single().expect("Failed to convert to Local DateTime");

            outputstring.push_str(&format!("{}\n{}\n", path.display().to_string().replace(" ", "%20"), local_date_time.format("%Y-%m-%d %H:%M:%S %z")));
        } else if path.is_dir() {
            outputstring.push_str(&format!("{}\n", &lastmod(path.to_str().unwrap(), input)));
        }
    }
    outputstring.replace(input, "")
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
                    Err(_why)      => {
                        // println!("failed to read file: {}", why); "".to_string();
                        continue
                    }
                };
                let compact_path = entry.path().to_str().unwrap().split("partials").last().unwrap().to_string().chars().skip(1).collect::<String>();
                //println!("{}",compact_path);
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

    if config.settings.backlinks.unwrap_or(false) {
        generate_backlinks(&all_content, &config, &mut links, &mut backlinks, &mut infos);
    }
    
    // now compile .md or .markdown files
    compile_markdown(&all_content, &config, &theme_html, &links, &backlinks, &infos, &partials, true);

    let theme_files = walk_directory(&theme_path, &config, &theme_path);

    copy_theme_files(&theme_files, &config, &theme_path);

    let _ = fs::copy("blazeconfig.toml", config.build.output.clone() + "/blazeconfig.toml");

    let universal_path = "blaze/universal".to_string(); 

    match fs::read_dir(&universal_path) {
        Ok(_) => (),
        Err(why) => panic!("error reading universal: {}", why)
    }

    let universal_files = walk_directory(&universal_path, &config, &universal_path);

    copy_theme_files(&universal_files, &config, &universal_path);

    if config.deployment.vercelcleanurl.unwrap_or(false) {
        // graphing time (only backlinks cause i dunno how to do forward links)
        // probably would've been better to save as not a hashmap but oh well
        let verceljson = String::from("{
  \"cleanUrls\": true
}");

        let verceljsonname = "output/versel.json";

        let mut verceljsonfile = File::create(verceljsonname).expect("Failed to create file");

        verceljsonfile.write_all(verceljson.as_bytes()).expect("Failed to write to file");
    }

    if config.settings.graph.unwrap_or(false) {
        let globalgraphdata = fs::read_to_string("output/global.json").expect("REASON");
        let trimmedString = globalgraphdata.trim_start_matches('{').trim_end_matches('}');
        let objects: Vec<&str> = trimmedString.split("}{").collect();

        let mut nodesObjects: Vec<String> = Vec::new();
        let mut linksObjects: Vec<String> = Vec::new();
        let _linksArray: Vec<String> = Vec::new();

        for string in objects {
            let sections: Vec<&str> = string.split(", ").collect();
            if let Some(nodesSection) = sections.iter().find(|s| s.starts_with("\"nodes\":[")) {
                nodesObjects.push(nodesSection.replace("\"nodes\":[", "").replace("]",""));
            } else {
                println!("Nodes section not found");
            }    

            if let Some(linksSection) = sections.iter().find(|s| s.starts_with("\"links\":[")) {
                linksObjects.push(linksSection.replace("\"links\":[", "").replace("]",""));
            } else {
                println!("Links section not found");
            }
        }

        let modifiedNodes = nodesObjects.concat().replace("}{", "}, {").replace("},{", "}, {");
        let modifiedLinks = linksObjects.concat().replace("}{", "}, {").replace("},{", "}, {");
        
        let outputstring = "{\"nodes\":[".to_owned() + &modifiedNodes + "], \"links\":[" + &modifiedLinks + "]}";
        let _ = fs::write("output/global.json", outputstring);
    }

    println!("Blaze has finished compiling");


    // probably a bad implementation since there's a lastmod thing above, fix later!

    let result = lastmod(&config.build.input, &config.build.input).lines()
                            .filter(|line| !line.trim().is_empty())
                            .collect::<Vec<&str>>()
                            .join("\n");

    let mut file = File::create("output/lastmod.txt")
        .expect("Failed to create file");

    file.write_all(result.as_bytes())
        .expect("Failed to write to file");
}
