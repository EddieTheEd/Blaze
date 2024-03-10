## To make a custom theme:

1. Add to your frontmatter the "theme" property, and then the theme name. e.g.:

```
---
title: Cool page
description: A cool page.
theme: mycustomtheme
---
```

2. Open the directory for your general theme in Blaze, e.g. the default directory. Open(or create) the themes folder and then create a css file with the exact name as the theme name in your frontmatter.

For the example above, if I was using the 'default' theme, I would put my custom css (relative to the Blaze repository root) at "blaze/themes/default/themes/mycustomtheme.css".

(i.e. in the same folder as this file!)

Ensure all properties here have the !important tag. If they are not loading properly, then try moving the {{theme}} string below your css files in the \_theme.html/head.html file. Generally, you want your {{theme}} string to be loaded last.
