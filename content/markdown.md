---
title: Markdown processing
description: Example of how markdown is processed.
---

this is some basic text.

*this is italicised text*

**this is bolded text**

# This is an h1 header

## This is an h2 header

### This is an h3 header

#### This is an h4 header

##### This is an h5 header

###### This is an h6 header

I like to shop. Here is a shopping list.
- Milk
- Sugar
- Methanol
- Flour

Here is a list of cool people.
1. John
2. John
3. *John*
4. **John**
5. ~~the impostor from amongus~~

[this is a markdown link to cool math games](https://www.coolmathgames.com/)

[this is a link to the index file](index)

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