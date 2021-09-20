# Model

Counter example part:

```rust
// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model { counter: i32 }
```

<details>
<summary>Example from a production app (this website)</summary>

```rust
pub struct Model {
    pub base_url: Url,
    pub page: Page,
    pub selected_seed_version: SeedVersion,
    pub guide_list_visibility: Visibility,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
    pub guides: Vec<Guide>,
    pub search_query: String,
    pub matched_guides: Vec<Guide>,
    pub mode: Mode,
}
```

</details>

---

- `Model` is the state (aka store, data, ...) of your application.

- In our case, it's a struct with a single [i32](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) field because we only need to track one value - the number of clicks.

- It can be almost anything -  a [type alias](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=alias#creating-type-synonyms-with-type-aliases), [enum](https://doc.rust-lang.org/book/ch06-00-enums.html), etc. Most `Model`s, however, are [structs](https://doc.rust-lang.org/book/ch05-00-structs.html) in real-world apps, and your `Model` has to be `static` (you can't save most [references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) in it).

## How to write a good `Model`

- It should be as simple as possible. If there is a way to derive data from the current `Model` instead of extending it (i.e. adding more fields), you should derive it, even at the cost of a small reduction in performance.

- Try to only save data in your `Model` - i.e. add field `students: Vec<Student>` instead of `manager: SchoolManager`, where `manager` contains `Vec<Student>` and a thousand other things. Exceptions include Seed-related items like handles and DOM elements, which will be discussed in later chapters. 

- Don't make your life unnecessarily hard.
  - Don't make your `Model` [generic](https://doc.rust-lang.org/book/ch10-00-generics.html).
  - Don't implement any `Model` methods (not even [default](https://doc.rust-lang.org/std/default/trait.Default.html) - see the detailed explanation at the end of the section).

- Your `Model` should be [the single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth) - i.e. add one field `selected_menu_item: MenuItemId` into your main `Model` instead of creating many instances of the component `MenuItem`, where each instance has own `Model` with field `selected: bool`.

- Try to be as expressive as possible and make impossible business rules unrepresentable in code by encoding them with the Rust type system.

   - Try to reduce the number of `bool`s and `Option`s to a minimum.

   - When you see multiple fields with the same simple type (`bool`, `String`, `u32`, `Option`, etc.) in your `Model`, you should try to remodel it.

   - I recommend reading the book [Domain Modeling Made Functional](https://fsharpforfunandprofit.com/books/) or at least [The "Designing with types" series](https://fsharpforfunandprofit.com/series/designing-with-types.html).

- When you need to create custom types that are used in the `Model`, write them below the `Model`. The *"children below the parent"* rule is valid for all nested structures. Example:
```rust
// ------ ------
//     Model
// ------ ------

struct Model {
  a_field: AType,
}

// ------ ATYPE ------

enum AType {
  AVariant
}

```
<details>
<summary>Why children below the parent?</summary>

Imagine some code with this pattern:
```
ChildA
impls for ChildA
ChildB
ChildC
..
Parent
```
In this pattern, you don't know which children are interesting for you because you don't know how and where they are used until you see also the parent.

Human short-term memory can hold only cca 7 items - that means it's very easily overloaded by reading child definitions and as a result the reader will start to jump between children and the parent to empty space and decrease cognitive load.

You can improve DX by moving children below the parent to allow readers to filter interesting children.

Another reason is scanning - readers (especially advanced developers) scan the code and try to recognize familiar patterns or basic building blocks - blocks like
```rust
// ------ ------
//     xxxxx
// ------ ------

xxxxxx Model / init / .. {
```
effectively work as checkpoints for the eyes.

</details>

<details>
<summary>Why not to implement <code>Default</code> and other standard traits</summary>

All implementations of standard traits (e.g. `From`, `Into`, `Default`, `Display`) are generally very useful only if the item (`struct`, `enum`...) is used in multiple contexts or with multiple other items - the generalization then makes sense because it implies that you are writing idiomatic Rust, and it plays nicely with other standard traits and items.  However, when you start to implement standard traits for many items, your codebase slowly turns into a sea of `.into()`, `::default()`, `.to_string()`, etc.

As the result:

   - You lose expressive domain-specific names, so it can be pretty hard to orient yourself when reading the code.
   - It bloats the code because standard traits have to cover many cases, so they tend to be more complicated.
   - You need to "bend" some parts of your code so they can be written with those implemented traits, which also makes the code harder to read and likely slows it down.

**`Default` trait**: We assume `xx::default()` calls are pretty cheap operations (see `Default` for [primitive types](https://doc.rust-lang.org/src/core/default.rs.html#132) or [Vec](https://doc.rust-lang.org/src/alloc/vec.rs.html#2334-2339)) - in most cases there isn't even memory allocation on the heap, and you probably won't find more expensive operations in `Default` implementations for other items. When you write more sophisticated `Default` code for your item and somebody uses this item in a nested structure, he will be very surprised once he writes some benchmarks.

One Seed user was even able to accidentally write a recursive loop of nested complex `Default`s that was causing stack overflows.

In Seed apps, you only need to instantiate `Model` once, so when you implement `Default` for `Model`:
  - It only bloats the code.
  - You'll have a tendency to overwrite default values in the `init` function because some `Model` parts will depend on `Url` or other values which results in worse readability and slower code. (You'll learn about `init` and `Url` in later chapters.)
  - It sends the misleading signal that `Model` is/can be created on multiple places.

</details>


## Single source of truth VS Components

A standard Seed app usually contains one app (aka root or main) `Model`, several page `Model`s and a few component `Model`s. It's often pretty clear where you should save data, and it's simple enough to keep the most of your models in your head during development. Due to this, there usually aren't problems with data synchronization or introducing unnecessary models (aka local states).

It's already possible, however, to integrate Javascript [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components) (we plan to support Rust Web Components as well) and [React-like Hooks](https://reactjs.org/docs/hooks-overview.html#state-hook) ([Seed Hooks](https://seed-style-hooks.netlify.app/hooks_home)) are waiting to be integrated into Seed. It will become pretty easy to create local states (even implicitly when you are using a component library), so here are some tips how to define your `Model`s:

1. Your business data should be kept in standard Seed `Model`s - ideally in the app model or in page models.

1. You should use state hooks for pure GUI data - e.g. a state variable `mouse_over` for a `button` component when you want only to switch the button's color on hover.

1. You can use Web Components for complex GUI elements or when you often need to interact with JS library / component.

(Don't worry, you'll learn more about Web Components and pages in later chapters- take look at [custom_elements](https://github.com/seed-rs/seed/tree/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/examples/custom_elements) and [pages](https://github.com/seed-rs/seed/tree/d514b2131a9e94f5ffe965f3d0ac74763a11aeb6/examples/pages) examples if you're brave enough.)
