# Msg

Counter example part:

```rust
// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ScrollToTop,
    ToggleGuideList,
    HideGuideList,
    ToggleMenu,
    HideMenu,
    SearchQueryChanged(String),
    ToggleMode,
    SwitchVersion(SeedVersion),
}
```

</details>

---

- `Msg`s are sent when something interesting has happened (e.g. the user has clicked a button) and your app should respond in some way (e.g. a value in your `Model` should be increased).

- You can also send `Msg` by calling `orders.send_msg(Msg::MyMessage)`

- `Msg` has similar limitations like `Model` - it can be almost anything, however the most `Msg`s are [enums](https://doc.rust-lang.org/book/ch06-00-enums.html) in real-world apps. And `Msg` has to be `static` (it basically means that you can't send the most messages that contain [references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing)).

(We'll talk about sending and handling messages in next chapters.)

## How to write a good `Msg`

- All `Msg` variants should be as simple as possible and hold only data or ideally nothing.
   - Example: `enum Msg { MenuItemClicked(MenuItemId), Save }`

- If you can choose the type for your ids (e.g. `MenutItemId`), pick rather [Uuid](https://docs.rs/uuid/0.8.1/uuid/struct.Uuid.html) than [String](https://doc.rust-lang.org/std/string/struct.String.html) or [u32](https://doc.rust-lang.org/std/primitive.u32.html). `Uuid` implements [Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html) and allows you to create entities in your frontend app before they are sent to your server.

- Don't make your life unnecessary hard.
   - Don't make your `Msg` [generic](https://doc.rust-lang.org/book/ch10-00-generics.html).
   - Don't implement any `Msg` methods.

- Try to be as expressive as possible - reduce the number of simple types (`bool`, `String`, `u32`, `Option`, etc.) to a minimum. Even type aliases are a huge improvement for readability.

- There are basically two types of messages. Try to follow the naming conventions:
  - Commands - e.g. `ScrollToTop`, `ToggleMenu`, `RemoveItem(ItemId)`, etc.
  - Events - e.g. `ButtonClicked`, `UrlChanged(subs::UrlChanged)`, `TextUpdated(String)`, etc.

## Attribute `derive`

> `#[derive(Copy, Clone)]`

We already know attribute `allow` from the previous chapters. 

However the most used is probably attribute `derive`. I recommend to read official documentation, especially these parts:
- [Adding Useful Functionality with Derived Traits](https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits)
- [Appendix C: Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#appendix-c-derivable-traits)
- [Clone and Copy for Duplicating Values](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#clone-and-copy-for-duplicating-values)
- [Trait std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)

**Notes from the front line:**

- Derive `Copy, Clone` for all items, where possible. It will make your life easier and your users and Clippy will appreciate it, too.
  - Some `derive` values are order-sensitive because they are sequentially applied to the code below them. That's why the alphabetical order in `derive(..)` is not important and by convention `Copy` should be always before `Clone` for better code scannability.

- There are edge-cases where you derive `Clone`, compiler is ok with it but you have still problems with calling `my_item.clone()`. In the most cases you can resolve it by implementing `Clone` manually. ([Related Rust issue](https://github.com/rust-lang/rust/issues/26925))

- Derive `Debug` at least for public items, if possible. Users will be able to fix the most of bugs in their apps and you'll receive more meaningful bug reports. Seed has macro `log!(item_a, item_b);` that you can use instead of `println!` to log things that implement `Debug` into the browser console log.

- Derive `Default` when you are sure it's really useful. If you decide to implement `Default` manually, make it super simple.

- `Eq` and `PartialEq` is often useful for simple enums. It allows you to use `==` as the more readable alternative to [pattern matching](https://doc.rust-lang.org/book/ch06-00-enums.html) and macros like [matches!](https://doc.rust-lang.org/beta/std/macro.matches.html). Example:

```rust
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}
...
   IF!(model.menu_visibility == Hidden => C.hidden)
   // VS
   IF!(matches!(model.menu_visibility, Hidden) => C.hidden)
   // or
   if let Hidden = model.menu_visibility { Some(C.hidden) } else { None }
```
(You'll learn more about Seed macro `IF!` in next chapters; `C.` is a typed CSS class container used in [seed-quickstart-webpack](https://github.com/seed-rs/seed-quickstart-webpack).)
