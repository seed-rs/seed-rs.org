# Prerequisites

## Rust
Proficiency in Rust isn't required to get started.
It helps, but I think you'll be able to build a usable webapp using only this guide.
For business logic beyond the GUI, more study may be required.
The official [Rust Book](https://doc.rust-lang.org/book/index.html) is a good
place to start. Especially-revelant chapters are `Understanding Ownership` `Using Structs`,
and `Enums and Pattern Matching`.

You should be able to get by with just the basic Rust syntax common to most programming
languages, eg conditionals, equalities, iteration, collections - and how Rust's borrow system applies
to strings. A skim through the first few chapters of the Book, and the examples here should provide 
what you need. Rust's advanced and specialized features like lifetimes, generics, smartpointers, and traits
aren't required to build an interactive GUI. If you run into a syntax or borrowing/ownership
problem, the compiler will often recommend a fix.

## Web fundamentals
Experience building websites using HTML/CSS or other frameworks
is required. Neither this guide nor the API docs describes how web pages are structured,
or what various DOM elements, attributes, styles etc do. You'll need to know these before
getting started. Seed provides tools to assemble and manipulate these fundamentals.
Mozilla's [MDN web docs](https://developer.mozilla.org/en-US/docs/Learn)
is a good place to learn about them.

## Other frontend frameworks
The design principles Seed uses are similar to those
used by React and Elm. If you're familiar with how to set up interactive web pages
using these tools, you'll likely have an easy time learning this.
