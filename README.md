# RipGrep Explorer

Explore the source code of your favorite projects using this TUI wrapper for [RipGrep](https://github.com/BurntSushi/ripgrep).

It launches a `rg` instance with arguments defined by the user, and adds the `--json` flag so it can parse the output of `rg`
command and creates a data structure with the parsed result. Take a look at this example

# Example

Clone this project, go to the root path of this project on your local machine and type this command on your terminal:

> cargo run -- fn

This will build the application and run it by passing to the executable the argument `fn`. This sole argument is the search term.
rg_explorer will find inside this project all files that have the `fn` string inside. As you already know, in Rust the `fn` keyword
is used to declare your functions. So the results will contain all rust source code files as result. rg_explorer will parse the output
of rg command and will create its own data structure: `RipGrep`. `RipGrep` has a special field called `Nodes`. This is the meat of
the application. On the "Home" page displayed right after entering the above command, you will see something like this

![image](https://user-images.githubusercontent.com/3003032/211956504-e51fb40f-e68d-4cc7-992c-e6db94a6ca07.png)

This is showing you the actual ripgrep command that rg_explorer has launched as child process. As you can see on the image, it is telling
ripgrep to:
- search_term = fn
- format of output is JSON
- Show 1 line of context after each match (`-A 1` flag)
- Show 1 line of context before each match (`-B 1` flag)
- Search inside the current folder (.)

To see the nodes hit the "n" key. You will go to "Nodes" page and will see something like this:

![image](https://user-images.githubusercontent.com/3003032/211955283-0672d97c-51a4-4a1a-91b6-b900e36e744a.png)

At the left side, you will see all the files inside the project that have the `fn` string inside. You can select one of them by hitting the 
up/down arrow key. The content of the "Detail" widget will show you each line with a match, surrounded by 1 lines of context after and 1 line
of context before.

The rest of the application... you are invited to rg explore it!

# Contribution

See the "Issues" tab of this github project to see what you can do. There are some issues labeled with [Good first issue](https://github.com/KarlHeitmann/rg_explorer/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3A%22good+first+issue%22)
that you can try. If you have some ideas, check out the discussion tab of this project

# Acknowledgement, inspiration.

- This [nice tutorial](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/) helped me get bootstraped with rust TUI library 
- In loving memory of Viktor Slüsarenko Stachniw. My best teacher at the Universidad Técnica Federico Santa María. Requiescat in pace, Maestro.

![DSC01349](https://user-images.githubusercontent.com/3003032/211959416-b3cf77d4-2837-49a7-8fdf-713851f205c6.jpeg)

- [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set)

- ¡AVE CHRISTUS REX! Iesus Dominus Est.
