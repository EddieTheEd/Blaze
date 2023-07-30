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
3. John
4. John
5. ~~the impostor from amongus~~


~~no strikethrough yet :(~~

added

==what about highlighted text? although its not conventional markdown? no? sadge==

WILL THIS UPDATE?!?!?!?

lets try that again, while its not updating

***it is here!!!!!*** yay

Here are some links:
[myself](showcase) 
[back to the index](index) 
[subfile](subfolder/subfile)  
[subsubfile](subfolder/subsubfolder/subsubfile)  

Here is a funny gif.
![funny gif](caption.gif)  
a funny gif

Here is some $\text{inline latex: } e^{i\pi} - 1 = 0$
- "\\ \\ \\" is ignored. (imagine the slashes are together, no spaces)

Below is a block of latex.
$$
\begin{align} 
\text{among us} \\ 
\text{in real life??!?!?}
\end{align}
$$
- *caveat: you must write "\\ \\ \\" to actually get a "\\ \\" in the output* (imagine the slashes are together, no spaces)
- *this has been fixed!! (hopefully)*

Here is the block of latex being used for something other than math:

$$
\begin{align}
n = \frac{m}{M}, \therefore n(Na_{2}CO_{3}) = \frac{6}{2\times 22.99 + 12.01 + 3\times 16} =5.66\times 10^{-2}mol \ (3SF)\\
c = \frac{n}{V}, \ \therefore [Na_{2}CO_{3}] = \frac{5.66\times 10^{-2}}{0.500}=1.13 \times 10^{-1} molL^{-1} \ (3SF) \\
n = cV, \therefore n (\text{reacting} Na_{2}CO_{3}) = 1.13\times 10^{-1} \times 0.020 = 2.26 \times 10^{-3}mol \ (3SF) \\
Na_{2}CO_{3} + 2HCl \to 2Na^{+} + CO_{2} + H_{2}O + 2Cl^{-} \\
\therefore \text{1:1 ratio, }n(HCl) = 2\times n(Na_{2}CO_{3}) = 4.53\times 10^{-3} mol \ (3SF) \\
c = \frac{n}{V}, \therefore [HCl] = \frac{4.53\times 10^{-3}}{0.0212}=0.2 molL^{-1} \ (1SF)
\end{align}
$$

The latex should be implemented thanks to Mathjax.

Now here is `some inline code!!!`

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

Wanna see my callout?


> [!info] Sample callout
> This is a callout? !!!! Notice how there isnt a line break here.
> Here is some markdown in the callout: *wowzies*, **wow**
> ## i also like headers in my callouts. this is a h2 header
> why? who knows.
>
> can you do line breaks? why yes you can! just add another ">"


That was a cool callout.

Wanna see my table?

here

| Heading 1 | Heading 2 |
| --------- | -------------- | 
| Item 1 | Item 2 |
| Item 3 | Item 4 |

tada!

anyways, here are some special cases of links.

obsidian likes the following links
[obbyindex](index.md)

==attempted highlight here==

~~also highlight is overwritten by bold and italics uh oh~~ **highlight just doesnt work here?!??!? oh thankfully strikethrough does**

Time for the big test for the table.

| Characteristic                        | Description                                                                                                                                                                                                                                                            |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Perfectionism                         | Such as pointing out how a person or their work is inadequate. Instead, expect that everyone will make mistakes and that mistakes offer opportunities for learning.                                                                                                    |
| Sense of urgency                      | Such as prioritising quick or highly visible results that can exclude potential allies. Instead, discuss what it means to set goals of inclusivity and diversity, particularly in terms of timing.                                                                     |
| Defensiveness                         | Such as spending energy trying to protect power or defend against charges of racism. Instead, work on your own defensiveness and understand the link between defensiveness and fear.                                                                                   |
| Valuing quantity over quality         | Such as directing organisational resources towards measurable goals. Instead, develop a values statement which expresses the ways in which you want to work, and make sure it is a living document that people apply to their daily work.                              |
| Worshipping the written word          | Such as valuing strong documentation and writing skills. Instead, work to recognise the contributions and skills that every person brings to the organisation.                                                                                                         |
| Believing in only one right way       | Such as concluding something is wrong with people who refuse to adapt or change. Instead, never assume that you or your organisation know what's best.                                                                                                                 |
| Paternalism                           | Such as decision-making processes that are only understood by those with power and unclear to those without it. Instead, include people who are affected by decisions in decision-making.                                                                              |
| Either/or thinking                    | Such as trying to simplify complex things. Instead, slow down, encourage people to do a deeper analysis, and sense that things can be both/and.                                                                                                                        |
| Power hoarding                        | Such as feeling threatened when anyone suggests organisational change. Instead, understand that change is inevitable and that challenges can be both healthy and productive.                                                                                           |
| Fear of open conflict                 | Such as equating the raising of difficult issues with being rude or impolite Instead, don't require those who raise difficult issues to do so in 'acceptable' ways, particularly if you're using the ways in which issues are raised as an excuse not to address them. |
| Individualism                         | Such as wanting individual recognition and credit. Instead, make sure credit is given to everyone who participates, not just the leaders.                                                                                                                              |
| Believing I'm the only one            | Such as thinking that if something is going to get done right, then 'I' have to do it. Instead, evaluate people based on their ability to delegate to others.                                                                                                          |
| Believing progress is bigger and more | Such as defining success as hiring more staff, developing more projects, or serving more people. Instead, make sure your goals speak to how you want to work, not just what you want to do.                                                                            |
| Believing in objectivity              | Such as considering emotions to be irrational and destructive to decision-making. Instead, push yourself to sit with discomfort when people express themselves in unfamiliar ways.                                                                                     |
| Claiming a right to comfort           | Such as scapegoating those who cause emotional or psychological discomfort. Instead, welcome discomfort as much as you can and understand that it is the root of all growth and learning.                                                                                                                                                                                                                                                                       |

Woops i forgot to test images.

Here is an image

![](assets/yes.png)

good good all works