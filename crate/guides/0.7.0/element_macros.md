# Element Macros

Element macros (`div!`, `img!`, etc.) represent HTML elements in the Seed world, e.g.:
```rust
div![
    C!["counter"],
    "This is a counter.",
]
``` 
becomes
```html
<div class="counter">
    This is a counter.
</div>
```

They are only slim wrappers for [`Node<Msg>`](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/node.rs#L18) - e.g. `let node: Node<Msg> = div![];` is a valid code. There is no "black magic" and you can even use `Node` directly instead of element macros, although it's not recommended.

_Note:_ There aren't any differences between elements that use opening & closing tags (`<div>..</div>`) or only self-closing tags (`<img>`; aka "void elements") - you just write:
```rust
img![
    attrs!{At::Src => "my_image.png"}
]
```

## UpdateEl

 You can throw all items that implement trait [UpdateEl](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/update_el.rs) into element macros. 
 
 It's implemented for:
   
- Seed-specific items like attributes, event handlers, nodes and DOM references. (We'll cover them in other chapters.)

- Some basic Rust types like strings and numbers. (Please [create an issue](https://github.com/seed-rs/seed/issues/new) if something important is missing.)

- Containers like [Option](https://doc.rust-lang.org/std/option/enum.Option.html), [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) and all [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)s that contain items that also implement `UpdateEl`.

- References to items that implement `UpdateEl`.

You can implement `UpdateEl` trait for your items - see implementation for [Button](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/component_builder/src/button.rs#L247-L253) in the example `component_builder`.

## How to write a good element macro invocation

- Write it with suitable brackets - `div![]`. In general:
  - Square `[]` are good for array-like macros - [`vec![]`](https://doc.rust-lang.org/std/macro.vec.html), `div![]`, `C![]` 
  - Curly `{}` for object-like macros - [`hashmap!{}`](https://docs.rs/maplit/1.0.2/maplit/macro.hashmap.html), `attrs!{}`
  - Round `()` for function-like macros and helpers - [`println!()`](https://doc.rust-lang.org/std/macro.println.html), `log!()`, `IF!()`

- If it's too long, split it into new `view` functions.

- Follow this pattern for inner item order:

```rust
div![
    // 1. Classes
    C!["a_class"],

    // 2. DOM references
    el_ref(element_reference),

    // 3. Element keys
    el_key(&element_key),

    // 4. Style (aka CSS)
    style!{St::Display => "flex"},

    // 5. Other attributes
    attrs!{At::Width => px(50)},

    // 6. Text content (string, numbers, ..)
    "Text",

    // 7. Event handlers
    ev(Ev::Click, |_| Msg::Clicked),

    // 8. Children
    h1!["Title"],
    things.iter().map(view_thing),
]
```

## Special Element macros & Helpers

<details>
<summary><code>svg!, raw!, md!, custom!, ..</code></summary>

## `svg!`

[SVG: Scalable Vector Graphics](https://developer.mozilla.org/en-US/docs/Web/SVG)

```rust
svg![
    rect![
        attrs! {
            At::Fill => card.bg_color,
        },
    ],
    circle![
        attrs! {
            At::Stroke => card.fg_color,
        },
    ],
]
```
becomes 
```html
<svg xmlns="http://www.w3.org/2000/svg">...</svg>
```

## `plain!`

`plain!` creates text `Node` from `Into<Cow<'static, str>>`.

It's rarely used because element macros create text `Node`s automatically:
```rust
div![ "I'll be a text node, hooray!", "Me too!" ]
```
However it's useful outside of element macros:
```rust
if let Data::Loaded(data) = data {
    view_data(data)  // returns `Node<Msg>`
} else {
    plain!["Loading..."]
}
```

 
## `empty!`
`empty![]` represents, well, nothing. It's useful in conditions like:
```rust
div![
    if menu.is_visible() {
        view_menu()
    } else {
        empty![]
    }
]
```
to satisfy compiler. However such conditions introduce a lot of boilerplate. To improve readability we recommend to use Seed macro `IF!`:
```rust
div![
    IF!(menu.is_visible() => view_menu())
]
```
`IF!` syntax/signature is: `IF!(predicate: bool => value: T) -> Option<T>`

## `raw!`

`raw!` creates `Vec<Node>` from `&str`.
```rust
div![
    raw!("<h1>Title</h1>"),
    // Inline `content.html` during compilation.
    raw!(include_str!("../content.html")),  
]
```

## `md!`

`md!` parses [Markdown](https://en.wikipedia.org/wiki/Markdown) `&str` and then creates `Vec<Node<Msg>>` like `raw!`.
```rust
div![
    md!("# Markdown"),  
]
```
- It uses parser [pulldown-cmark](https://crates.io/crates/pulldown-cmark). All [Options](https://docs.rs/pulldown-cmark/0.7.1/pulldown_cmark/struct.Options.html) are enabled so you can use all supported extensions - see [examples](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/markdown/md/examples.md). 
- Parsing long texts can be slow - in that case we recommend to convert `*.md` files to `*.html` files and include their content with `raw!` + `include_str!` during compilation - see example [markdown](https://github.com/seed-rs/seed/tree/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/markdown). (We plan to mitigate this and similar issues.) 

## `nodes!`

`nodes!` is basically `vec!` that accepts everything that implements `IntoNodes`.
  - In other words - Converts items to `Vec<Node<Ms>` and returns flattened `Vec<Node<Ms>`.

```rust
nodes![
    md!["# Hello"],
    h2!["world"],
    vec![
        div!["Do you like"],
        div!["Seed?"]
    ],
]
```

## `custom!`

`custom!` is useful for integrating [custom elements](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements).

```rust
custom![
    Tag::from("code-block"),
    attrs! {
        At::from("lang") => lang,
        At::from("code") => code,
    }
]
```

If you want to know how to write [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components) & custom elements with [LitElement](https://lit-element.polymer-project.org/) and then use these elements in your Seed app, look at example [custom_elements](https://github.com/seed-rs/seed/tree/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/custom_elements). (We plan to support also Rust Web Components once it's possible to write them.)

</details>

## Why element macros? (Detailed explanation)

<details>
<summary><b>Rust types</b> vs <b>Macros</b> vs <b>Templates</b></summary>

In an ideal world, we would encode all business rules and browser/libraries interfaces by Rust type system and let the compiler to warn us when we are breaking them. 

Although the Rust type system is very expressive and getting even better, it can't cover all possible rules. However Rust expressiveness is the smallest problem to resolve when you want to write web UI. The main problem is that we can't even define all rules. Why? Because of HTML+CSS+Javascript.

I think HTML+CSS+JS are "languages" with the steepest learning curve and it's pretty easy to accidentally break the code/layout even for very experienced developers. Why?
- No static types - only runtime errors. Errors are also often partially mitigated by the browser. Examples:
  - Browsers try to fix or hide invalid HTML.
  - Invalid CSS is ignored.
  - JS often only writes a cryptic error to console log and tries to continue.
  
  => How do you want to write/generate Rust types/interfaces? How do you catch runtime errors? (especially the ones caused by invalid/unsupported CSS/HTML/JS calls?)
  
- Browsers and even different versions of the same browser often support different features and they have different bugs.
  
  => Should we disable all unsupported and deprecated and buggy and experimental features in common browsers? What are common browsers? Which features?

  => _Fun fact_: We have dummy DOM call in the Seed core, because some browsers kill your Seed app without that call (it's reported bug).
- Inheritance everywhere - all elements are highly context dependent. Some elements can be containers for some other specific elements. Some values are inherited. And remember that there are no official types and there are also [custom elements](https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements)...

  => Rust doesn't like inheritance very much. And you just can't get types from custom JS element.

- Have you tried to write [accessible](https://developer.mozilla.org/en-US/docs/Learn/Accessibility/What_is_accessibility) website with [semantically correct HTML](https://html.com/semantic-markup/)?

  => Are the rules that enforce accessibility and semantic markup possible to define? Or are they content-dependent and should be defined by human per case?

- API inconsistency. Some element attributes can be set declaratively through HTML and they are accessible also through JS properties. Some attributes are used only to set default values. Some properties can't be set through attributes. Some input elements are set through attribute value, some through their content. Some attribute values are only valid if another attribute has a specific value. Some attributes don't work if the element has been attached programatically. And again, good luck with custom elements...
  
  => There are many workarounds in Seed to make the browser built-in elements work. And there were also the most bugs. 
  
  => I have no idea how to find out all rules and encode them by a type system.

- Weird naming and behavior. Do you know that SVG is case-sensitive (unlike HTML)? _"..And offsetHeight. Can an offset even have height?.."_ - see the rest of the article in [Elm's Browser.Dom description](https://package.elm-lang.org/packages/elm/browser/latest/Browser.Dom).

  => Do we really want to write cumbersome Rust types because of bad API/markup? 

I assume you understand now why there isn't a fully-typed HTML/DOM/Browser API wrapper in Seed.

Element macros are more suitable for this because they are "typed enough" - the most of code is still checked by the compiler, however it allows you to write very declarative and readable code. As the result, it's much easier to find bugs in your code while you are looking into HTML spec/MDN docs and scanning the code.

And what about templates / template engines?
  - Why do you want to throw out Rust expressive type system and IDE help?
  - Why do you want to learn a new syntax and cryptic error messages? And force your users to do the same?
  - Why do you want to add another dependency with potential bugs and increase app size?

</details>

## Future & Alternatives

- You'll be able to create own element macros - it's useful for custom elements and for writing components.

- We'll write experimental layout/component libraries once [Seed Style](https://seed-style-hooks.netlify.app/home) (new typed CSS) and [Seed Hooks](https://seed-style-hooks.netlify.app/hooks_home) (better alternative to React Hooks) are integrated.

- You can write own typed components - see examples [component_builder](https://github.com/seed-rs/seed/tree/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/component_builder) and [tea_component](https://github.com/seed-rs/seed/tree/3134d21c6fcb2383685885687fe2a7610fb2ff74/examples/tea_component).

- There are libraries that use element macros or `Node`s under the hood (e.g. [Savory](https://gitlab.com/MAlrusayni/savory)).
