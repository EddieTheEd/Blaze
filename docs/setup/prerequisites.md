---
title: Prerequisites
description: What things you need to set up so that it plays nice with Blaze.
---

Before you start using Blaze, you probably need to set up a few other things first.

## Obsidian
Configure your Obsidian vault settings as follows. The most important setting is absolute pathing and no wikilinks. [^1]
![](../assets/obbysettings.png)
## Github
1. Most importantly of all, you *need a Github account*.
2. Once you have a Github account, you can go to the [Blaze github](https://github.com/EddieTheEd/Blaze) and click the `Code` and then clone it[^2], either through `HTTPS` or `SSH`. (It doesn't matter which.)
![cloning](../cloning.png)
3. Once it's downloaded, open your file explorer and find the repository's folder. It should like like this.[^3]
![](../assets/foldering.png)
4. Check that this is the right folder by opening it. It should look something like this.
![](../assets/foldering2.png)
## Adding content
### I have an existing obsidian vault
Say your files look something like this. (Your Obsidian vault doesn't literally have to be called `Obsidian Vault`)
![](../assets/foldering2-1.png)
All you need to do is simply move the `Obsidian Vault` folder into the `Blaze` folder. You can also delete the `docs` folder if you want, because that is for the [blaze.toomwn.xyz website](https://blaze.toomwn.xyz/).
![](../assets/foldering%20number%20whatever.png)
Next, you need to change the `blazeconfig.toml` file.
Go to the `build` section and change the string of the `input` option to **the name of your Obsidian Vault**. In this example, I would have to change it to `'Obsidian Vault'.`

![](../assets/rename%20input.png)

Now you're done! Open the new folder as a vault and your Obsidian stuff should be there.
### I want to make a new vault
1. Go into the Blaze folder, as above.
![foldering](../assets/foldering2.png)
2. Create a new folder, and name it to what you want your Obsidian vault to be called. For example, in the image below I have named mine `Obsidian Vault`.
![foldering number whatever](../assets/foldering%20number%20whatever.png)
3. Now open Obsidian[^4] and click the `Create` button, then navigate to the named folder inside the `Blaze` folder.
![](../assets/obsidian%20create.png)
4. Next, you need to change the `blazeconfig.toml` file. Go to the `build` section and change the string of the `input` option to **the name of your Obsidian Vault**. For example, I would have to change it to `'Obsidian Vault'.`
![](../assets/rename%20input.png)

Now you're done!

[^1]: This is because Blaze creates links relative to the file.. Also, Blaze is designed to **only** read [markdown links](https://www.markdownguide.org/basic-syntax/), while Obsidian tends to default to [wikilinks](https://en.wikipedia.org/wiki/Help:Link).
[^2]: If you are unaware how to clone repositories, see [here](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository).
[^3]: Note that this is [Thunar](https://docs.xfce.org/xfce/thunar/the-file-manager-window), Xfce's file manager. The font is [Georgia](https://en.wikipedia.org/wiki/Georgia_(typeface)).
[^4]: If you haven't installed it already [go here](https://obsidian.md/).