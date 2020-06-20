# View

 Visual layout (ie HTML/DOM elements) is described declaratively with [macros]( https://doc.rust-lang.org/book/ch19-06-macros.html) to simplify syntax. Each element
is represented by a macro, eg `div![]`. These act as functions that accept an arbitrary
number of parameters in any order. They handle parameters based exclusively on type.

The view is defined by a function passed to 
[App::build()](https://docs.rs/seed/0.6.0/seed/app/struct.App.html#method.build). It takes a `&Model`
as a parameter and outputs something that implements the ` View` trait, which is imported in the prelude.
Usually, this is a `Node`, or `Vec<Node>`, representing all nodes that will be inserted as children
on the top-level one. The top-level `Node` exists in the DOM as specified by
[AppBuilder::mount()](https://docs.rs/seed/0.6.0/seed/app/builder/struct.Builder.html#method.mount) and defaults to an element with id `app`.
 It may composed into sub-functions, which can be thought of like components in other frameworks. 

Examples:
```rust
fn view(model: &Model) -> Node<Msg> {
    h1![ "Let there be light" ]
}
```

```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        h1![ "Let there be light" ],
        h2![ "Let it be both a particle and a wave" ],
    ]
}
`````
In either of those examples, you could use the signature: `fn view(model: &Model) -> impl View<Msg>` instead.
This allows you to change between them without changing the function signature.

[TODO]: # (Explain what `Msg` type parameter means and how it's connected to `update` function)

## The Node Enum
The Virtual DOM is represented by nested [Nodes](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/enum.Node.html).
`Node` has 3 variants: 

- `Text` holds a [Text](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/text/struct.Text.html)
struct. Mostly for internal use, but can be created with `Node::new_text()`.
- `Element` wraps an [El](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/el/struct.El.html), which is
the main component of our VDOM. Created using macros, described below.
- `Empty` is a placeholder that doesn't render anything; useful in conditional/ternary logic.
Created using the `empty![]` macro, or `seed::empty()`.


## Elements, attributes, styles
Elements are created using macros, named by the lowercase name of
each element, and imported into the global namespace. Eg `div!` above. We use this code to import them:
```rust
use seed::{*, prelude::*};
```

These macros accept any combination of the following parameters. Each can be omitted, included once,
or included multiple times.
- [Attrs](https://docs.rs/seed/0.6.0/seed/virtual_dom/attrs/struct.Attrs.html) structs
- [Style](https://docs.rs/seed/0.6.0/seed/virtual_dom/style/struct.Style.html) structs
- [Listener](https://docs.rs/seed/0.6.0/seed/virtual_dom/event_handler_manager/listener/struct.Listener.html) structs, which handle events
- `String`s or `&str`s representing a node text
- [Node](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/enum.Node.html) structs, representing a child
- `Vec`s of `Attrs`, `Style`, `Listener`, or `Node`
- `Map`s of `Attrs`, `Style`, `Listener`, or `Node`, ie the result of `map()`,
 without having to explicitly `collect`

The parameters can be passed in any order; the compiler knows how to handle them
based on their types. Children are rendered in the order passed.

Views are described using [El](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/el/struct.El.html) structs.

`Attrs` and `Style` are thinly-wrapped hashmaps created with their own macros: `attrs!{}` and `style!{}`
respectively.

Example:
```rust
fn view(model: &Model) -> impl View<Msg> {
    let things = vec![ h4![ "thing1" ], h4![ "thing2" ] ];
    
    let other_things = vec![1, 2];

    div![ attrs!{At::Class => "hardly-any"}, 
        things,  // Vec<Node<Msg>
        other_things.map(|t| h4![t.to_string()]),  // Map
        h4![ "thing3?" ],  // El
    ]
}
```
Note that you can create any of the above items inside an element macro, or create it separately,
and pass it in. You can separate different items by comma, semicolon, or space.

Keys passed to `attrs!` can be `Seed::At`s, `String`s, or `&str`s. 
Keys passed to `style!` can be `Seed::St`s, `String`s, or `&str`s.
Values passed to `attrs!`, and `style!` macros can 
be owned `Strings`, `&str`s, or for `style!`, `unit`s. 

You use the `unit!` macro to apply units. There's a `px` function for the
special case where the unit is pixels:
```rust
style!{St::Width => unit!(20, px);}
style!{St::Width => px(20);}  // equivalent
```

Some types, like `Option`s, implement a trait allowing them to be used directly in
`style!`:
```rust
let display: &str = "flex";
let direction: String = "column".to_string();
let order: Option<u32> = None;
let gap: Option<&str> = Some("8px");

let style = style![
    St::Display => display,
    St::FlexDirection => direction,
    St::Order => order,
    St::Gap => gap,
];
```

We can set multiple values for an attribute using 
[Attribute.add_multiple](https://docs.rs/seed/0.6.0/seed/virtual_dom/attrs/struct.Attrs.html#method.add_multiple).
 This is useful for setting multiple classes. Note that we must set this up outside of
the view macro, since it involves modifying a variable:
```rust
fn a_component() -> Node<Msg> {
    let mut attributes = attrs!{};
    attributes.add_multiple(At::Class, vec!["A-modicum-of", "hardly-any"]);

    div![ attributes ]
}
```

Seed validates attributes [against this list](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes);
The [At](https://docs.rs/seed/0.6.0/seed/dom_entity_names/enum.At.html) 
enum includes only these values, and `&strs` passed are converted into `At`s. If you
wish to use a custom attribute, use 
[At::Custom](https://docs.rs/seed/0.6.0/seed/dom_entity_names/enum.At.html#variant.Custom)
, eg `At::Custom(name)`, where `name` is a `String` of your
attribute's name. In `attrs!` when using `&str`s, inserting an unrecognized attribute
will do the same. Similar `Custom` methods exist for 
[Style](https://docs.rs/seed/0.6.0/seed/dom_entity_names/enum.St.html#variant.Custom),
[Namespace](https://docs.rs/seed/0.6.0/seed/browser/dom/namespace/enum.Namespace.html#variant.Custom),
[Tag](https://docs.rs/seed/0.6.0/seed/dom_entity_names/enum.Tag.html#variant.Custom), and
[Event](https://docs.rs/seed/0.6.0/seed/dom_entity_names/enum.Ev.html#variant.Custom).

The `class!` and `id!` convenience macros allow settings
attributes as a list of classes, or a single id, if no other attributes are required.
Do not mix and match these with each other, or with attrs!; all but the last-passed
will be thrown out.
```rust
fn a_component() -> Node<Msg> {
    // ...
    span![ class!["calculus", "chemistry", "literature"] ],
    span![ id!("unique-element") ],
    // ...
}
```

You can conditionally add classes with the `class!` macro:
```rust
let active = true;

class![
    "blue",
    "highlighted" => active,
    "confusing" => 0.99999 == 1
    
]
```

Styles and Attrs can be passed as refs as well, which is useful if you need to pass
the same one more than once:
```rust
fn a_component() -> Node<Msg> {
    let item_style = style!{
        St::MarginTop => px(10);
        St::FontSize => unit!(1.2, em)
    };

    div![
        ul![
            li![ &item_style, "Item 1", ],
            li![ &item_style, "Item 2", ],
        ]
    ]
}
```

For boolean attributes that are handled by presence or absence, like `disabled`, `checked`,
`autofocus` etc, use 
[.as_at_value](https://docs.rs/seed/0.6.0/seed/virtual_dom/values/trait.AsAtValue.html#tymethod.as_at_value):
 `input![ attrs!{At::Disabled => false.as_at_value() ]`:

```rust
fn a_component() -> Node<Msg> {
    // ...
    input![ attrs!{At::Type => "checkbox"; At::Checked => true.as_at_value()} ]
    input![ attrs!{At::AutoFocus => true.as_at_value()} ]
    // ...
}
```
`At::Checked => true.as_at_value()` is equivalent to the presence of a `checked` attribute,
and `At::Checked => false.as_at_value()` is equivalent to omitting it.

To change Attrs or Styles you've created, edit their .vals HashMap. To add
a new part to them, use their .add method:
```rust
let mut attributes = attrs!{};
attributes.add(At::Class, "truckloads");
```

Example of the style tag, and how you can use pattern-matching in views:
```rust
fn view(model: &Model) -> impl View<Msg> {
    div![ style!{
        St::Display => "grid";
        St::GridTemplateColumns => "auto";
        St::GridTemplateRows => "100px auto 100px"
        },
        section![ style!{St::GridRow => "1 / 2"},
            header(),
        ],
        section![ attrs!{St::GridRow => "2 / 3"},
            match model.page {
                Page::Guide => guide(),
                Page::Changelog => changelog(),
            },
        ],
        section![ style!{St::GridRow => "3 / 4"},
            footer()
        ]
    ]
}
```

We can combine Attrs and `Style` instances using their 
[merge](https://docs.rs/seed/0.6.0/seed/virtual_dom/attrs/struct.Attrs.html#method.merge)
 methods, which take
an `&Attrs` and `&Style` respectively. This can be used to compose styles from reusable parts. 
Example:
```rust
fn a_component() -> Node<Msg> {
    let base_style = style!{"color" => "lavender"};

    div![
        h1![ &base_style.merge(&style!{St::GridRow => "1 / 2"}) "First row" ],
        h1![ &base_style.merge(&style!{St::GridRow => "2 / 3"}) "Second row" ],
    ]
}
```

Perhaps more cleanly, we can use multiple `Style`s together, to merge their entries:
```rust
fn a_component() -> Node<Msg> {
    let base_style = style!{"color" => "lavender"};

    div![
        h1![ 
            &base_style, 
            style!{St::GridRow => "1 / 2"},
            "First row" ],
        h1![ 
            &base_style, 
            style!{St::GridRow => "2 / 3"}, 
            "Second row" ],
    ]
}
```

You can also pass Vecs of Styles to the view macros. This may be convenient when building styles 
before passing to the view:
```rust
fn a_component(condition: bool) -> Node<Msg> {
    let mut styles = vec![style!{"color" => "lavender"}];
    
    if condition {
        styles.push(style!{St::FontSize: px(12)})
    }

    div![
        h1![ styles, "First row" ],
    ]
}
```


Overall: we leverage of Rust's strict type system to flexibly-create the view
using normal Rust code.


`El` has several helper methods which can be chained together:
```rust
let my_el = div![]
    .add_text("Words")
    .add_class("complete")
    .add_attr("alt".to_string(), "a description".to_string())
    .add_style(St::Height, "20px".to_string())
    .replace_text("Oops, not complete");

```

## Svg
You can create `SVG` elements in the same way as normal `Html` elements.
Setting the `xmlns` attribute isn't required; it's set automatically when using the macro.

Example using macros:
```rust
svg![
    rect![
        attrs!{
            At::X => "5",
            At::Y =>"5",
            At::Width => "20",
            At::Height => "20",
            At::Stroke => "green",
            At::StrokeWidth => "4",
        }
    ]
]
```

The same example using [from_html](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/el/struct.El.html#method.from_html):
```rust
Node::from_html(
r#"
<svg>
    <rect x="#5" y="5" width="20" height="20" stroke="green" stroke-width="4" />
</svg>
"#)
```

Another example, showing it in the `View` fn:
```rust
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        svg![
            attrs!{
                At::Width => "100%";
                At::Height => "100%";
                At::ViewBox => "0 0 512 512";
            },
            path![ 
                attrs!{
                    At::Fill => "lightgrey";
                    At::D => "M345.863,281.853c19.152-8.872,38.221-15.344,56.1"  // etc
                }
            ],
            // More elements as required, eg mesh, polyline, circle
        ]
    ]
}
```

## Canvas (unreleased; for now, you can use `web_sys` directly.

Seed provides helper functions for use with `Canvas`:
```rust
fn draw() {
    let canvas = seed::canvas("canvas").unwrap();
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
    draw();
}
```


##  Components
The analog of components in frameworks like React are normal Rust functions that that return
[Nodes](https://docs.rs/seed/0.6.0/seed/virtual_dom/node/enum.Node.html).
These functions take parameters that are not treated in a way equivalent
to attributes on native DOM elements; they just provide a way to 
organize your code. In practice, they're used in a way similar to components in React.

For example, you could organize one of the examples in the Structure section of the guide like this:
```rust
    fn text_display(text: &str) -> Node<Msg> {
        h3![ text ]
    }  
    
    div![ style!{St::Display => "flex"; St::FlexDirection => "column"},
        text_display("Some things"),
        button![ simple_ev("click", Msg::SayHi), "Click me!" ]
    ]
```

The text_display component returns a single `Node` that is inserted into its parents'
`children` Vec; you can use this in patterns as you would in React. You can also use
functions that return `Vec`s of`Node`s, which you can incorporate into other `Node`s
using normal Rust code. See the Fragments section below. Rust's type system
ensures that only `Node`s  can end up as children, so if your app compiles,
you haven't violated any rules.
 
Unlike in JSX, there's a clear syntax delineation between natural DOM
elements (element macros), and custom components (function calls): We called text_display
above as `text_display("Some things")`, not `text_display![ "Some things" ]`.

## Fragments
Fragments (`<>...</>` syntax in React and Yew) are components that represent multiple
elements without a parent. They're useful to avoid
unnecessary divs, which clutter the DOM, and breaks things like tables and CSS-grid. 
There's no special fragment syntax: have your component return a `Vec` of `Node`s instead of 
one. Add it to the parent's element macro:
```rust
fn cols() -> Vec<Node<Msg>> {
    vec![
        td![ "1" ],
        td![ "2" ],
        td![ "3" ]
    ]
}

fn items() -> Node<Msg> {
    table![
        tr![ cols() ]
    ]
}
```

You can mix `Node` `Vec`s with `Node`s in macros:
```rust
fn items() -> Node<Msg> {
    // You may wish to keep complicated or dynamic logic separate.
    let mut more_cols = vec![ td![ "another col" ], td![ "and another" ] ];
    more_cols.push(td![ "yet another" ]);

    table![
        tr![
            td![ "first col" ],  // A lone element
            cols(),  // A "fragment" component.
            td![ "an extra col" ], // A element after the fragment
            // A Vec of Els, not in a separate func
            vec![ td![ "another col" ], td![ "and another" ] ],
            more_cols  // A vec of Els created separately.
        ]
    ]
}
```

## Dummy elements
When performing ternary operations inside an element macro, all
branches must return an `Node` (Or `Vec` of `Node`s) to satisfy Rust's type system. Seed provides the
[empty](https://docs.rs/seed/0.6.0/seed/fn.empty.html) function, which creates a `Node` that will not be 
rendered, and its `empty![]` macro alias, which is more concise and consistent:
```rust
div![
    if model.count >= 10 { h2![ style!{St::Padding => 50}, "Nice!" ] } else { empty![] }
]
```
