# Models & Msgs

Let's define `Model`s and `Msg`s for individual pages and then include them into the root `Model` and `Msg`. We'll ignore page header - we'll resolve it with the root `Model` and `Msg`.

I recommend to look at page designs while you are writing your `Model`s.

## Home

We don't need a `Model` for Home page at all. And if we want to show different buttons according to user state (anonymous / logged in) we would use the root `Model` or a shared `Model` derived from the root one.

The only active control on the page is the button "Go to Time Tracker". It will be a link so we don't need `Msg` at all.

---

## Login / Registration

It will be handled by our identity provider. Things like JWT token will be stored in another location - e.g. in a shared/root `Model`.

---

## User Management

It will be also handled by our identity provider. 

---

## Clients & Projects

```rust
type ClientId = Ulid;
type ProjectId = Ulid;
```

```rust
struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

    clients: RemoteData<BTreeMap<ClientId, Client>>,
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

struct Client {
    name: String,
    projects: BTreeMap<ProjectId, Project>,
}

struct Project {
    name: String,
}
```

```rust
enum Msg {
    ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,
    
    // ------ Client ------

    AddClient,
    DeleteClient(ClientId),

    ClientNameChanged(ClientId, String),
    SaveClientName(ClientId),
    
    // ------ Project ------

    AddProject(ClientId),
    DeleteProject(ClientId, ProjectId),
    
    ProjectNameChanged(ClientId, ProjectId, String),
    SaveProjectName(ClientId, ProjectId),
}
```

You don't find a text like "Changes saved at 19:32:36" in the design however we should add it to signal user that everything works and all data are safely stored on the server. The related data will be stored in the `Model` field `change_status`.

When some saving requests to backend fail - e.g. when the server is down - an error message should be displayed. We can store error messages in the `Model` field `errors`. The user can clear all error messages by click the button - this action fires `Msg::ClearError`.

We've chosen the same container for our entities like in the previous TodoMVC example - a combination of `BTreeMap` + `Ulid` as an id. However this time we want to render items from the newest to the oldest ones. It isn't a problem, we can use [reverse iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev) because `BTreeMap` implements [DoubleEndedIterator](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html).

We can define [new types](https://doc.rust-lang.org/stable/rust-by-example/generics/new_types.html) for `ClientId` and `ProjectId` however type aliases should be good enough in this case - the risk of the wrong argument order is relatively small thanks to logical and consistent parameter order. I think it's a good trade-off for eventual boilerplate.

There are often pairs `ClientId` + `ProjectId`. It's a trade-off between some boilerplate in `Msg` variants and more complex algorithms or structures that would allow us to find the chosen entity in the tree. We can refactor it once we need deeply nested entities / tree.

The app synchronizes Client and Project names with the `Model` on each change / key press in the corresponding HTML elements - we will listen for [input event](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event). However we don't want to send a request to back-end on each key press. One possible solution is to implement some kind of [debouncing](https://css-tricks.com/debouncing-throttling-explained-examples/) for input events. Or we can try just listen for [change event](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event). `Msg::SaveClientName` or `Msg::SaveProjectName` would be sent on `change event`.

`fetch::Result<BTreeMap<ClientId, Client>>` is just an alias for `Result<BTreeMap<ClientId, Client>, FetchError>`, where `FetchError` is imported by `seed::prelude::*`. (We'll talk about fetching in the next chapters.)

`DateTime`, `Local`, `Duration`, etc. will be imported from the crate [chrono](https://crates.io/crates/chrono). 

---

## Time Tracker

```rust
type ClientId = Ulid;
type ProjectId = Ulid;
type TimeEntryId = Ulid;
```

```rust
struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

    clients: RemoteData<BTreeMap<ClientId, Client>>,
    timer_handle: StreamHandle, 
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

struct Client {
    name: String,
    projects: BTreeMap<Ulid, Project>,
}

struct Project {
    name: String,
    time_entries: BTreeMap<Ulid, TimeEntry>,
}

struct TimeEntry {
    name: String,
    started: DateTime<Local>,
    stopped: Option<DateTime<Local>>,
}
```

```rust
enum Msg {
    ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,
    
    Start(ClientId, ProjectId),
    Stop(ClientId, ProjectId),

    DeleteTimeEntry(ClientId, ProjectId, TimeEntryId),
    
    TimeEntryNameChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryName(ClientId, ProjectId, TimeEntryId),
    
    TimeEntryStartedChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryStarted(ClientId, ProjectId, TimeEntryId),

    TimeEntryDurationChanged(ClientId, ProjectId, TimeEntryId, String),
    
    TimeEntryStoppedChanged(ClientId, ProjectId, TimeEntryId, String),
    SaveTimeEntryStopped(ClientId, ProjectId, TimeEntryId),

    OnSecondTick,
}
```

We need to update active TimeEntry on each second so the user see the current time. When the user clicks the button "Stop", the current time will be saved to the active TimeEntry field `stopped`.

While the user is editing the TimeEntry duration, `stopped` is automatically recomputed. When editing is done, `Msg::SaveTimeEntryStopped` is fired. The user can edit the duration and `stopped` time only when the TimeEntry is inactive.

`timer_handle` is a "pointer" to a timer that fires `Msg::OnSecondTick` on each second - it's a `Msg` [Stream](https://docs.rs/futures/0.3.5/futures/stream/trait.Stream.html). The timer is disabled and removed when `timer_handle` is [dropped](https://doc.rust-lang.org/std/ops/trait.Drop.html). `StreamHandle` is imported by `seed::prelude::*`.

---

## Time Blocks

```rust
type ClientId = Ulid;
type TimeBlock = Ulid;
```

```rust
struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

    clients: RemoteData<BTreeMap<ClientId, Client>>,
}

enum RemoteData<T> {
    NotAsked,
    Loading,
    Loaded(T),
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

struct Client {
    name: String,
    time_blocks: BTreeMap<Ulid, TimeBlock>,
    tracked: Duration,
}

struct TimeBlock {
    name: String,
    status: TimeBlockStatus,
    duration: Duration,
    invoice: Option<Invoice>,
}

enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

struct Invoice {
    custom_id: Option<String>,
    url: Option<String>,
}

```

```rust
enum Msg {
    ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>),
    ChangesSaved(Option<FetchError>),
    ClearErrors,

    // ------ TimeBlock ------
    
    AddTimeBlock(ClientId),
    DeleteTimeBlock(ClientId, TimeBlockId),
    SetTimeBlockStatus(ClientId, TimeBlockId, TimeBlockStatus),

    TimeBlockDurationChanged(ClientId, TimeBlockId, String),
    SaveTimeBlockDuration(ClientId, TimeBlockId),

    // ------ Invoice ------

    AttachInvoice(ClientId, TimeBlockId),
    DeleteInvoice(ClientId, TimeBlockId),

    InvoiceCustomIdChanged(ClientId, TimeBlockId, String),
    SaveInvoiceCustomId(ClientId, TimeBlockId),

    InvoiceUrlChanged(ClientId, TimeBlockId, String),
    SaveInvoiceUrl(ClientId, TimeBlockId),
}
```

We hope that our back-end will be able to compute `tracked` time from Client's TimeEntries.

---

## Settings

```rust
struct Model {
    changes_status: ChangesStatus,
    errors: Vec<FetchError>,

    form: Form,
}

enum ChangesStatus {
    NoChanges,
    Saving { requests_in_flight: usize },
    Saved(DateTime<Local>),
}

struct Form {
    username: String,
    email: String,
    password: String,
    confirm_password: String,

    errors: FormErrors,
}

struct FormErrors {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    confirm_password: Option<String>,
}
```

```rust
enum Msg {
    ChangesSaved(Option<FetchError>),
    ClearErrors,

    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    ConfirmPasswordChanged(String),

    Save,
    DeleteAccount,
}
```

We'll make the logic simpler and check that the Username and Email are available after the user clicked the button "Save". 

---

# Root Model & Msg

```rust
struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
    email: String,
}

enum Page {
    Home,
    ClientsAndProjects(page::clients_and_projects::Model),
    TimeTracker(page::time_tracker::Model),
    TimeBlocks(page::time_blocks::Model),
    Settings(page::settings::Model),
    NotFound,
}
```

```rust
enum Msg {
    UrlChanged(subs::UrlChanged),
}
```

`ctx` will be accessible from all pages - it's our "shared" state. The fields in `Context` and `User` will probably change while we will be integrating the identity provider.

The app drops the previous page `Model` when routing to the another page. It's not always the best option but it's the simplest and the most predictable way to switch pages. (If you don't want to drop `Model`s, see the example [pages_keep_state](https://github.com/seed-rs/seed/tree/8d04fcde8a22f785fa20d28cb2f1a9b3b2d7e790/examples/pages_keep_state).)

We should be able to represent all header buttons as links - we need only one `Msg` variant `UrlChanged`.

---

All our `Model`s and `Msg`s are defined!

You've probably noticed there are some repeating parts among our `Model`s and `Msgs` like `RemoteData`, `ChangesStatus` or `errors` - fight the urge to refactor it and create an abstraction now. Let's leave it decoupled because we are still not able to fully recognize all patterns and predict all future implementation details that may break our abstractions.

In the next chapter we'll setup the project with a basic file structure to accommodate our pages.
