# New features in v0.6.0:
Reference [this PR](https://github.com/seed-rs/seed/pull/330).

- Added the `nodes!` enum. This vec-like enum can be used to combine different collections
of `Node`s, for use in view macros.
Example:
```rust
nodes![
    h1!["a node"],
    vec![
        h4["a vec"],
        h4["of nodes"],
    ],
]
```

- You can use native DOM elements safely because 
`ElRef::get` checks if the referenced element exists, 
is in the DOM now, and has the right type. Check ouf the `canvas`, `user_media`,
or `todomvc` examples.

- `El::custom` added; this can be used to create an Element with a custom / arbitrary tag.


# Quickstart

## Setup

The two fundamental things to setup:

 * [Rust](https://www.rust-lang.org/tools/install), the language itself.
 * Run `rustup update` from the cli: the project assumes a relatively new version of Rust.
 * `cargo install cargo-make`, a broadly useful tool to for Rust [workflows](https://github.com/sagiegurari/cargo-make).


## The theoretical minimum

The two main steps:

 * [Generate](https://github.com/seed-rs/seed-quickstart/generate) your own repo from this one as a template, then clone it.
 * From the clones root-directory: `cargo make start` - builds the project and serves it to [localhost:8000](http://127.0.0.1:8000).

#### Also worth noting...

`cargo make watch` is much like running `start`, which is automatically re-run when a Rust file is changed.

The project [Makefile](https://github.com/seed-rs/seed-quickstart/blob/master/Makefile.toml) has other usefull commands, such as release builds.


## A little deeper

Alternatively, create a new lib with Cargo: `cargo new --lib appname`. Here and everywhere it appears in this guide, `appname` should be replaced with the name of your app.

If not using the quickstart repo, create an Html file with a body that contains this:

```html
<section id="app"></section>
<script type="module">
    import init from '/pkg/package.js';
    init('/pkg/package_bg.wasm');
</script>
```

The first line above is an empty element with id: It's where your app will render.
The subsequent ones load your app's wasm modules.

The quickstart repo includes this file. You will eventually need to modify it to
change the page's title, add a description, favicon, stylesheet etc.

`Cargo.toml`, which is a file created by Cargo that describes your app, needs `wasm-bindgen`, `web-sys`, and `seed` as dependencies, and crate-type
of `"cdylib"`. The version in the quickstart repo has these set up already. Example:

```toml
[package]
name = "appname"
version = "0.1.0"
authors = ["Your Name <email@address.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
seed = "^0.6.0"
wasm-bindgen = "^0.2.50"
```

## A short example

Here's an example demonstrating structure and syntax; it can be found in working form
in the [counter example](https://github.com/seed-rs/seed/tree/master/examples/counter).
Descriptions of its parts are in the
Guide section below. Its structure follows [The Elm Architecture](https://guide.elm-lang.org/architecture/).

_lib.rs_:

```rust
use seed::{*, prelude::*};

// Model

struct Model {
    count: i32,
    what_we_count: String
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            what_we_count: "click".into()
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// How we update the model
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
    }
}


// View

/// A simple component.
fn success_level(clicks: i32) -> Node<Msg> {
    let descrip = match clicks {
        0 ..= 5 => "Not very many 🙁",
        6 ..= 9 => "I got my first real six-string 😐",
        10 ..= 11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃"
    };
    p![ descrip ]
}

/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> impl View<Msg> {
    let plural = if model.count == 1 {""} else {"s"};

    // Attrs, Style, Events, and children may be defined separately.
    let outer_style = style!{
            St::Display => "flex";
            St::FlexDirection => "column";
            St::TextAlign => "center"
    };

    div![ outer_style,
        h1![ "The Grand Total" ],
        div![
            style!{
                // Example of conditional logic in a style.
                St::Color => if model.count > 4 {"purple"} else {"gray"};
                St::Border => "2px solid #004422"; 
                St::Padding => unit!(20, px);
            },
            // We can use normal Rust code and comments in the view.
            h3![ format!("{} {}{} so far", model.count, model.what_we_count, plural) ],
            button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
            button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],

            // Optionally-displaying an element
            if model.count >= 10 { h2![ style!{St::Padding => px(50)}, "Nice!" ] } else { empty![] }
        ],
        success_level(model.count),  // Incorporating a separate component

        h3![ "What are we counting?" ],
        input![ attrs!{At::Value => model.what_we_count}, input_ev(Ev::Input, Msg::ChangeWWC) ]
    ]
}


#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
```

For a truly minimimal example, see [lib.rs in the quickstart repo](https://github.com/seed-rs/seed-quickstart/blob/master/src/lib.rs)

## Building and running

To build your app, run `cargo make build`, and to host on a dev server, run `cargo make serve`.

For a more robust starting setup, check out Martin Kavik's [seed-quickstart-webpack repo](https://github.com/seed-rs/seed-quickstart-webpack).

## Running included examples

To run an example located in the [examples folder](https://github.com/seed-rs/seed/tree/master/examples),
run `cargo make start example_name`, where you replace `example_name` with the example name. Eg:
`cargo make start counter`.

Some examples also require to run API server in another terminal window - `cargo make start_server example_name`.

When server(s) are running, open [127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

## Resources
- [Awesome-seed-rs](https://github.com/seed-rs/awesome-seed-rs): A curated list of resources.
- [Seed Realworld](https://github.com/seed-rs/seed-rs-realworld): A detailed realworld example site.
- [Engineering Rust Web Applications](https://erwabook.com/): A book describing full-stack Rust web-development, using Seed for the frontend.
