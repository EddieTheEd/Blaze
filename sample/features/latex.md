---
title: Latex
description: An implementation of Latex that uses MathJax.
---

Latex works thanks to the general implementation of [MathJax](https://www.mathjax.org/), with block latex being rendered by [Rust's Markdown parser](https://docs.rs/markdown/1.0.0-alpha.11/markdown/struct.Constructs.html#structfield.math_flow).

Here is some $\text{inline latex: } e^{i\pi} - 1 = -2$, which can still be used within a normal line. For example, $\text{this text is rendered by MathJax}$, while this text isnt.

Below is a block of latex.
$$
\begin{align} 
\text{among us} \\ 
\text{in real life??!?!?}
\end{align}
$$

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

$$\begin{align}
\text{Cl_{2(aq)}} \text{ half-cell}: \\
\text{Greenish-yellow, pungent effervescent is bubbled into a colourless solution} \\
\text{Loses its effervescence} \implies \text{ colourless solution} 
\end{align}$$
