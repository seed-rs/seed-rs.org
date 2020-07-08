# View

Counter example part:

```rust
// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
pub fn view(base_url: &Url) -> Node<Msg> {
    div![
        C![C.mt_32, C.flex, C.justify_center,],
        div![
            C![
                C.text_2xl,
                // sm__
                C.sm__text_4xl,
                // lg__
                C.lg__text_6xl,
            ],
            div![C![C.font_bold,], "404",],
            div![C![C.my_12,], "Page not found"],
            a![
                C![
                    C.block,
                    C.text_right,
                    C.text_green_500,
                    C.hover__underline,
                    C.hover__text_green_700,
                ],
                attrs! {
                    At::Href => Urls::new(base_url).home()
                },
                "Home"
            ],
        ],
    ]
}
```

</details>

---

- `view` is the function that basically transforms `Model` to HTML.

- It's invoked by Seed when the app should be rerendered - the typical scenario is:
  1. An action happens - e.g. user clicks on a button.
  1. Seed calls your `update` function.
  1. Seed schedules rerender.
  1. The browser notifies Seed that it's the right time for the render (see [requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)).
  1. Seed calls your `view` function.
  1. Seed rerenders the app.

- It's also invoked after the `init` function to render the app for the first time.

## `view` function signature

> `fn view(model: &Model) -> Node<Msg>`

- **&Model** is [immutable reference](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) to prevent you from mutating the state directly. It's possible to break this "limitation" with [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value) but we strongly oppose to do that, because `view` invocations are pretty unpredictable and such `Model` changes would be surprising and hard to find. (However there will be a way how to preserve and modify local `view` variables once [React-like Hooks](https://seed-style-hooks.netlify.app/hooks_home) are integrated.)

- [Node](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/node.rs#L13-L22) represents HTML element or text used as an element content. It's usually created by _element macros_ like `div!` or `span!`. (You'll learn about _element macros_ in next chapters.)

- Your `view` function can return everything that implements [IntoNodes](https://github.com/seed-rs/seed/blob/3134d21c6fcb2383685885687fe2a7610fb2ff74/src/virtual_dom/node/into_nodes.rs). It means your `view` signature can be also `fn view(model: &Model) -> impl IntoNode<Msg>` but we DON'T recommend this form because it's not very expressive and it makes chaining of nested `view`s harder. The most used and preferable forms are `-> Node<Msg>` or `-> Vec<Node<Msg>>`.

- In our Counter example, there is also another Clippy attribute `trivially_copy_pass_by_ref`. Clippy is sad because the `Model` in Counter example is too simple so it doesn't make sense to pass it into the `view` function by reference because copying it would be more efficient. Clippy won't bother you with it once your `Model` doesn't implement `Copy`.

## How to write a good `view`

- Don't hesitate to split `view` into helpers/"sub-views". If these helpers return nodes, name them `view_*` - e.g. `view_footer` or `view_menu_item`.

- Root `view` and all nested `view_*` functions should have in their function signature as the output value either `Node<Msg>` or `Vec<Node<Msg>>`. They can also return corresponding alternatives with `Option`.

- Pass only necessary `Model` variables into `view_*` functions.

- When you need to write some helpers, respect the rule *"children below the parent"* as always.

`view` is the most complex app part => you'll find information and best practices for things like elements, attributes and event handlers in next chapters.

## Future

- `view` and `view_*` functions will have own local state once [Seed Hooks](https://seed-style-hooks.netlify.app/hooks_home) are integrated.

- I'll maybe try to write GUI designer that generates `view` in the distant future. Or at least a plugin for [Figma](https://www.figma.com/) or [Adobe XD](https://www.adobe.com/uk/products/xd.html). It should make development more enjoyable & faster and make cooperation between developers and designers smoother. (Don't hesitate to write [me](https://github.com/MartinKavik) your opinions/ideas.)
