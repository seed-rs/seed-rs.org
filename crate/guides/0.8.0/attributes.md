# Attributes

Attribute macros `C!`, `style!` and `attrs!` represent HTML element attributes in the Seed world, e.g.:
```rust
div![
    C!["counter", IF!(selected => "active")],
    style!{
        St::Display => "flex",
        St::Padding => px(10),
    },
    attrs!{At::Title => "A Title"},
    "This is a counter.",
]
``` 
becomes
```html
<div class="counter active" title="A Title" style="display:flex;padding:10px">
    This is a counter.
</div>
```

# C!

- Macro `C!` accepts all items that implement trait [ToClasses](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/to_classes.rs). 
- If you use multiple `C!` invocations in one element, then the classes will be merged. (This rule is valid for all attributes.)

`ToClasses` is implemented for `String` and `&str`, references and containers `Option` and `Vec`.

Example of some valid input type combinations:
```rust
let selected = false;
let optional_classes: Option<Vec<String>> = None;
div![
    C!["counter", IF!(selected => "active")],
    C![IF!(true => vec!["class_a", "class_b"])],
    C![optional_classes],
]   
```
Corresponding HTML:
```html
<div class="counter class_a class_b"></div>
```

<details>
<summary>Why C!?</summary>

The name `C` breaks Rust naming conventions (macros should be written in `snake_case!`), but it's a trade-off for better scannability (you can distinguish element macros and attribute macros on the first glance). And it will be consistent with future names of other entities (e.g. `A.` for other attributes and `E.` for event handlers).
</details>

_Note:_ If you want to use [Tailwind CSS](https://tailwindcss.com/) and typed classes, look at [seed-quickstart-webpack](https://github.com/seed-rs/seed-quickstart-webpack).

# style!

`style!` expects key-value pairs, where:
  - **Key** is a CSS property name - e.g. `St::Display`
    - You can also use custom property names - e.g. `St::from("custom_name")`

  - **Value** can be everything that implements [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html) and it can be wrapped in `Option`.
    - There are [helpers](https://github.com/seed-rs/seed/blob/master/src/browser/dom/css_units.rs#L97-L144) for CSS units.

Example of some valid input type combinations:
```rust
let selected = true;
let apply_custom = true;
div![
    style!{
        St::Margin => px(50),
        St::MaxWidth => unit!(50, %),
        St::Top => 0,
        St::Padding => px(20) + " " + &px(15)
        St::BackgroundColor => if selected { "green" } else { "white" },
        St::from("custom_name") => IF!(apply_custom => "a_value"),
    }
]   
```
Corresponding HTML:
```html
<div style="
    margin:50px;
    max-width:50%;
    top:0;
    padding:20px 15px;
    background-color:green;
    custom_name:a_value
"></div>
```

## Future

Macro `style!` will be replaced with [Seed Style](https://seed-style-hooks.netlify.app/home). 

[Seed Style](https://seed-style-hooks.netlify.app/home) is basically typed inline CSS on steroids. It allows you to write also pseudo-classes like `:hover` directly in your Rust elements so it eliminates the need to write standalone style files. And there are many more useful features.

# attrs!

`attrs!` expects key-value pairs, where:
  - **Key** is an attribute name - e.g. `At::Title`
    - You can also use custom attribute names - e.g. `At::from("custom_name")`

  - **Value** can be [AtValue](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/values.rs#L67-L88) or  everything that implements [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html). `AtValue` has 3 variants:
    - `Ignored` - The whole attribute is ignored (i.e. not rendered to HTML at all). It's useful for _[boolean HTML attributes](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes)_ and in your conditions.
    - `None` - The attribute value is not used (i.e. rendered as empty string). It's also useful for _boolean HTML attributes_.
    - `Some(String)` - If `v` in `At::X => v`, implements `ToString`, then it's automatically transformed to `AtValue::Some(v)`.

_Note:_ `C!` and `style!` are basically only `attrs!`'s specializations - you can write 
```rust
attrs!{At::Class => "class_a", At::Style => "top:0"}
```
but it's not recommended.

## `as_at_value`

Method `as_at_value` is automatically attached to all Rust `bool`s. It helps with _boolean attributes_. It allows you to write:
```rust
let disabled = false;
...
attrs!{
    At::Disabled => disabled.as_at_value()
}
```
instead of:
```rust
attrs!{
    At::Disabled => if disabled { AtValue::None } else { AtValue::Ignored }
}
```
_Note:_ Without that `.as_at_value()` call, variable `disabled` would be only casted into `String` and rendered in HTML as `disabled="false"`. 

## `attrs!` example
```rust
let disabled = true;
div![
    attrs! {
        At::Disabled => disabled.as_at_value(),
        At::Title => "a_title",
        At::AutoFocus => AtValue::None,
        At::from("custom_name") => 123,
    }
]   
```
Corresponding HTML:
```html
<div disabled="" title="a_title" autofocus="" custom_name="123"></div>
```

## Future

Macro `attrs!` will be replaced with a safer and more readable API that consists struct `A` and associated methods. You can see drafts in [this issue](https://github.com/seed-rs/seed/issues/312#issuecomment-565832751).


