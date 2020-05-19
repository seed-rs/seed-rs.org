# Model

Counter example part:

```rust
// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
type Model = i32;
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

- `Model` is the state (aka store, data, ..) of your application.

- In our case it's only a [type alias](https://doc.rust-lang.org/book/ch19-04-advanced-types.html?highlight=alias#creating-type-synonyms-with-type-aliases) for [i32](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) because we only need to track one value - the number of clicks.

- It can be almost anything - type alias, [enum](https://doc.rust-lang.org/book/ch06-00-enums.html), etc. However the most `Model`s are [structs](https://doc.rust-lang.org/book/ch05-00-structs.html) in real-world apps. And `Model` has to be `static` (it basically means that you can't save the most [references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing) into it).

## How to write a good `Model`

- It should be as simple as possible. If there is a way how to derive data from the current `Model` instead of extending it (i.e. adding more fields), you should derive it, even at the cost of small reduction in performance.

- Try to save only data into your `Model` - i.e. add field `students: Vec<Student>` instead of `manager: SchoolManager`, where `manager` contains `Vec<Student>` and thousand other things. Exceptions are Seed-related items like handles and DOM elements (we will discuss them in other chapters).

- Don't make your life unnecessary hard.
  - Don't make your `Model` [generic](https://doc.rust-lang.org/book/ch10-00-generics.html).
  - Don't implement any `Model` methods.

- Your `Model` should be [the single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth) - i.e. add one field `selected_menu_item: MenuItemId` into your main `Model` instead of creating many instances of the component `MenuItem`, where each instance has own `Model` with field `selected: bool`.

- Try to be as expressive as possible and make impossible business rules unrepresentable in code by encoding them with Rust type system.

   - Try to reduce the number of `bool`s and `Option`s to a minimum.
   
   - When you see multiple fields with the same simple type (`bool`, `String`, `u32`, `Option`, etc.) in your `Model`, you should try to remodel it.
   
   - I recommend to read the book [Domain Modeling Made Functional](https://fsharpforfunandprofit.com/books/).


## Single source of truth VS Components

A standard Seed app usually contains one app (aka root or main) `Model`, several  page `Model`s and a few component `Model`s. It's often pretty clear where you should save data and it's simple enough to keep the most of your models in your head during development. So there usually aren't problems with data synchronization or introducing unnecessary models (aka local states).

However it's already possible to integrate Javascript [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components) (we plan to support also Rust Web Components) and [React-like Hooks](https://reactjs.org/docs/hooks-overview.html#state-hook) are waiting for integration into Seed. It means it will be pretty easy to create local states (even implicitly when you are using a component library), so there are some tips how to define your `Model`s:

1. Your business data should be kept in standard Seed `Model`s - ideally in the app model or in page models.

1. Then you should use state hooks for pure GUI data - e.g. a state variable `mouse_over` for `button` component when you want only to switch the button's color on hover.

1. And you can use Web Components for complex GUI elements or when you often need to interact with JS library / component.

(Don't worry, you'll learn more about Web Components and pages in other chapters.)
