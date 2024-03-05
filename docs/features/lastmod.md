---
title: Lastmod
description: Last modified date for md files.
---

As you'll see above, below the description is the date that the file was last modified, according to git. If you are running Blaze in a custom Github workflow, remember to change the checkout part to fetch the whole git history, else lastmod will only show the date of the most recent commit.

```
- name: Checkout code
    uses: actions/checkout@v3
    with:
      fetch-depth: 0
```
