---
title: Codeblocks
description: A demonstration of codeblocks
---

Here is `some inline code!!!`

Don't like inline code? Have a block of code:

```rust
// Here is some random rust code that ossac wrote.
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
```

have some css code

```css
.info-callout {
    background-color: var(--lightblue);
    padding: 8px;
    border-radius: 5px;
}
```

```html
<!--Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. Testing what happens when codeblocks overflow. -->

```

Enjoy! [^1]

[^1]: This is here to test codeblocks with sidenotes.
