---
title: Plan
description: The plan for Blaze's development
---

The plan will be divided into 4 parts:

## Spring Cleaning

*Polishing and reorganising the code so future work is easier*

- Remove unnecessary parts of CSS
- Create separate .js files for partials
- Properly load js scripts

## Crucial Elements

*These are the things needed for basic function. The benchmark for this is whether my obsidian vault works with as expected.*

- draft: true excluding files
- backlinks fixed
- Switch from Prism to [Rehype Pretty Code](https://rehype-pretty-code.netlify.app/) (thanks Jzhao)

## Awesome Features

*Really cool features that I want Blaze to have*

- Full text search
- Themes
- Analytics
    - This is a maybe. I dunno, I never felt that analytics were important
- Minification
    - Have js and css minified

## Customisation

*Making it usable for the user*

- Make it fully customisable from blazeconfig.toml
    - Conditional logic in the html partials
