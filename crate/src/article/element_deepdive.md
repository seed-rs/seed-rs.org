# Element-creation macros, under the hood

# This page is out of date. Standby.

For a better understanding of how views are created, reference the
[El API docs page](https://docs.rs/seed/0.2.0/seed/dom_types/struct.El.html).
The following code returns an `El` representing a few nested DOM elements displayed
in a flexbox layout:
```rust
    div![ style!{"display" => "flex"; "flex-direction" => "column"},
        h3![ "Some things" ],
        button![ "Click me!" ]
    ]
```

This declarative syntax is created using macros, which constrct `El`s from the arguments passed:
 The macros know how to use arguments based solely on their type.
If a String or &str is passed, it's stored as the El's `text` field.
`Attrs` and `Style` structs are stored as the `attrs` and `style` fields respectively.
`Listeners`, and Vecs of them are stored as the `listeners` field. The same principle applies
to `Els`, for the `children` field. `DidMount`, `DidUpdate`, and `WillUnmount` are also detected
appropriately, and passed into appropriate fields.

Here's an another way to construct the same nested `El` as above, using constructors
instead of macros. Reference the docs page for a full list of modifier methods. These
provide conveniet syntax over manually editing fields. (In most cases, you won't
edit `El`s at all; you'll create them declaratively using macros.)
```rust
use seed::dom_types::{El, Attrs, Style, Tag};

let mut heading = El::empty();
heading.set_text("Some things")

let mut button = El::empty(Tag::Button);
button.set_text("Click me!");

let mut elements = El::empty(Tag::Div);
elements.add_style("display", "flex");
elements.add_style("flex-direction", "column");
elements.children = vec![heading, button];

elements
```

The following equivalent example shows creating the required structs without constructors,
to demonstrate that the macros and constructors above represent normal Rust structs,
and provides insight into what abstractions they perform.

```rust
// We don't provide an example of a Listener: These are more complicated to 
use seed::dom_types::{El, Attrs, Style, Tag};

El {
    tag: Tag::Div,
    attrs: Attrs { vals: HashMap::new() },
    style: Style { 
        vals: hashmap_string!{
            "display" => "flex",
            "flex-direction" => "column"
        }
    },
    events: Events { vals: Vec::new() },
    text: None,
    children: vec![
        El {
            tag: Tag::H3,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: Some(String::from("Some things")),
            children: Vec::new()
            id: None,
            next_level: None,
            el_ws: None,
            namespace: None,
            did_mount: None,
            did_update: None,
            will_unmount: None,
        },
        El {
            tag: Tag::button,
            attrs: Attrs { vals: HashMap::new() },
            style: Style { vals: HashMap::new() },
            listeners: Vec::new();
            text: Some(String::from("Click me!")),
            children: Vec::new(),
            id: None,
            next_level: None,
            el_ws: None,
            raw_html: false,
            text_node: false,
            namespace: None,
            did_mount: None,
            did_update: None,
            will_unmount: None,
        } 
    ]
}
```
For most uses, the first example (using macros) will be the easiest to read and write.
You can mix in constructors in components as needed, depending on your code structure.
It's evident that struct literals are too verbose, due to the auxillary fields.