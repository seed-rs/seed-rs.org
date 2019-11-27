# Misc features

## Logging in the web browser
To output to the web browser's console (ie `console.log()` in JS), use 
 the `log!`, which is imported in the seed prelude: 
`log!("On the shoulders of", 5, "giants".to_string())`. You can use the `error!` macro
in a similar way, equivalent to JS's `console.error()`.

## Custom tags
Seed generally retricts the element tags allowed by using Enums for the tags, and
a predefined set of element-creation macros. If you wish to use a custom tag, you can
use using `Tag::from` (`El` and `Tag` are
exposed in the prelude), either with the `El::empty` constructor, or using the `custom!`
element-construction macro, where we pass our custom tag as an argument:
```rust
let mut custom_el = El::empty(Tag::Custom("mytag".to_string()));
custom_el.set_text("Words");

custom![ Tag::from("anothertag".into())
    custom_el,
]
```
An example is provided as part of the [window_events](https://github.com/David-OConnor/seed/tree/master/examples/todomvc)
example.

## Local storage
You can store page state locally using web_sys's [Storage struct](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Storage.html)

Seed provides convenience functions `seed::storage::get_storage`, which returns 
the `web_sys::storage` object, and `seed::storage::store_data` to store an arbitrary
Rust data structure that implements serde's Serialize. Example use:

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// ...
#[derive(Serialize, Deserialize)]
struct Data {
    // Arbitrary data (All sub-structs etc must also implement Serialize and Deserialize)
}

let storage = seed::storage::get_storage();
seed::storage::store(storage, "my-data", Data::new());

// ...

let loaded_serialized = storage.get_item("my-data").unwrap().unwrap();
let data = serde_json::from_str(&loaded_serialized).unwrap();
```

## Display markdown and raw HTML
Seed supports creating elements from markdown text, using [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)
internally. Use the [Node::from_markdown()](https://docs.rs/seed/0.1.6/seed/dom_types/struct.Node.html#method.from_markdown)
method to create an element that accepts a markdown &str as its only parameter, and displays
it normally as html. Note that it does not support syntax highlighting. You can render raw HTML with `El::from_html(html)`, where `html` is a 
&str of HTML. You can also use the `raw!` and `md!` macros for `from_html` and 
`from_markdown` respectively.

Example:
```rust
fn view(model: &Model) -> Vec<Node<Msg>> {

    let markdown = 
"
## Hello world

Let's set the existence-of-God issue aside for a later volume,
and just [learn to code](https://play.rust-lang.org/).
";

    let html = 
"
<div>
    <p>It is a truth universally acknowledged, that a single man in 
    possession of a good fortune, must be in want of a good time.</p>
</div>
";
    
    vec![
        Node::from_markdown(markdown)   // or md!(markdown)
        Node::from_html(html)  // or raw!(html)
    ]
}
```

This works for SVG as well:
```rust
fn view(model: &Model) -> Impl View<Msg> {
    Node::from_html(
r#"
<svg xmlns="http://www.w3.org/2000/svg">
    <rect x="5" y="5" width="20" height="20"></rect>
</svg>
"#)
}
```

## Using `web_sys` to view element data.
`web_sys`, which Seed uses internally, can be used to view information about elements. For example:
```rust
fn get_height(id: &str) -> i32 {	
    let html_el = seed::document().get_element_by_id("my_el").unwrap();
    let h = html_el.client_height();	
    log!("Height {:?}", h);	
    h	
}
```
Where we've given the element we wish to query id `my_el`.


## Some convenience functions
You can use `seed::document()` and `seed::window()` to access the `web_sys` document
and window functions. Example:
```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        button![ 
            simple_ev("click", Msg::Increment), 
            format!("Hello, World × {}", model.val),
            did_mount(|_| {
                seed::document().set_title("New title")
            })
        ]
    ]
}
```

Additionally, use `seed::html_document()` in the same way, to return a 
[HtmlDocument](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlDocument.html)

We also include `seed::canvas()`, and `seed::canvas_context()`. (Unreleased)

You can call `seed::cookies()` to retrieve all cookies from the current `HtmlDocument`.

## Input elements are controlled
`input`, `textarea`, and `select` elements are always controlled, in the vein of `React`'s
[controlled components](https://reactjs.org/docs/forms.html#controlled-components).
This means that even if there's no event associated with user input to these fields, their
value will always stay in sync with the model, which may mean ignoring text input if
not set up with a `Ev::Input` event.
