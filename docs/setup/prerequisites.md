---
title: Prerequisites
description: What things you need to set up so that it plays nice with Blaze.
---

Before you start using Blaze, you probably need to set up a few other things first.

## Obsidian

- Make sure that your Obsidian vault is generated as the **content folder**. What this means is that the .obsidian folder should be a direct *subdirectory* of the content folder.
- Configure your Obsidian vault settings as follows. The most important setting is absolute pathing and no wikilinks. [^1]

![](assets/obbysettings.png)

## Github

[^1]: This is because Blaze reads with the content directory as the root. Also, Blaze is designed to **only** read [markdown links](https://www.markdownguide.org/basic-syntax/), while Obsidian tends to default to [wikilinks](https://en.wikipedia.org/wiki/Help:Link).
