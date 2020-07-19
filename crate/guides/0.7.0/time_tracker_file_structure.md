# File Structure

We've finished the hardest parts - design and architecture. Let's celebrate it by writing some code!

Please create a new project based on [seed-quickstart](https://github.com/seed-rs/seed-quickstart) as usual.

The current file structure in the new project:
```
Cargo.toml
index.html
src/
    lib.rs

.. some other files and folders
```

and once we are done, it should look like:
```
Cargo.toml
index.html
src/
    lib.rs
    page.rs
    page/
         home.rs
         clients_and_projects.rs
         time_tracker.rs
         time_blocks.rs
         settings.rs
         not_found.rs
```

So let's update or create all the files!

## Cargo.toml

```toml
...
[dependencies]
chrono = "0.4.13"
ulid = "0.4.0"

...
```

We'll add more dependencies later, but those two should be enough to integrate our `Model`s into the app.

---

## index.html

```html
<head>
    ...
    <title>Time Tracker</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css">
</head>
```

We've added a website title and [Bulma](https://bulma.io/documentation/overview/start/)'s minimized CSS. We'll integrate Bulma's SASS and [Font Awesome](https://fontawesome.com/) later.

---

## src/lib.rs

<details>
<summary>Code</summary>

```rust
#![allow(clippy::wildcard_imports)]
// @TODO: Remove.
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

mod page;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::Home,
    }
}

// ------ ------
//     Model
// ------ ------

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

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {},
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div!["Root view"]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
```

</details>

_Note:_ The `page` module and other things aren't defined yet so we can't compile it. 

---

## src/page.rs

```rust
pub mod home;
pub mod clients_and_projects;
pub mod time_tracker;
pub mod time_blocks;
pub mod settings;
pub mod not_found;
```

---

## src/page/home.rs

```rust
use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
    div!["Home view"]
}
```

We don't need `Model` or `Context` (yet) => no `view` parameters. The same apply for `Msg` - we don't have it so we'll use the parent one which is hidden under the generic type `Ms`.

---

## src/page/clients_and_projects.rs

<details>
<summary>Code</summary>

```rust
use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use std::collections::BTreeMap;

type ClientId = Ulid;
type ProjectId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
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

pub struct Client {
    name: String,
    projects: BTreeMap<ProjectId, Project>,
}

struct Project {
    name: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
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

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClientsFetched(Ok(clients)) => {},
        Msg::ClientsFetched(Err(fetch_error)) => {},

        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},

        Msg::ClearErrors => {},

        // ------ Client ------

        Msg::AddClient => {},
        Msg::DeleteClient(client_id) => {},

        Msg::ClientNameChanged(client_id, name) => {},
        Msg::SaveClientName(client_id) => {},

        // ------ Project ------

        Msg::AddProject(client_id) => {},
        Msg::DeleteProject(client_id, project_id) => {},

        Msg::ProjectNameChanged(client_id, project_id, name) => {},
        Msg::SaveProjectName(client_id, project_id) => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["ClientsAndProjects view"]
}
```

</details>

_Note:_ `struct Client` is `pub` because the `Client` is used as a part of `ClientsFetched(fetch::Result<BTreeMap<ClientId, Client>>)`. Variants are automatically declared as `pub` when the `enum` is also `pub` in Rust. It's a warning now but it'll probably become an error in the future. You can find multiple discussions on this topic among Rust issues and on forums.  

---

## src/page/time_tracker.rs

<details>
<summary>Code</summary>

```rust
use seed::{prelude::*, *};

use chrono::prelude::*;
use ulid::Ulid;

use std::collections::BTreeMap;

type ClientId = Ulid;
type ProjectId = Ulid;
type TimeEntryId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
        timer_handle: orders.stream_with_handle(streams::interval(1000, || Msg::OnSecondTick)),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
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

pub struct Client {
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

// ------ ------
//    Update
// ------ ------

pub enum Msg {
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

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClientsFetched(Ok(clients)) => {},
        Msg::ClientsFetched(Err(fetch_error)) => {},

        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},

        Msg::ClearErrors => {},

        Msg::Start(client_id, project_id) => {},
        Msg::Stop(client_id, project_id) => {},

        Msg::DeleteTimeEntry(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryName(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryStartedChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryStarted(client_id, project_id, time_entry_id) => {},

        Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, name) => {},

        Msg::TimeEntryStoppedChanged(client_id, project_id, time_entry_id, name) => {},
        Msg::SaveTimeEntryStopped(client_id, project_id, time_entry_id) => {},

        Msg::OnSecondTick => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["TimeTracker view"]
}
```

</details>

---

## src/page/time_blocks.rs

<details>
<summary>Code</summary>

```rust
use seed::{prelude::*, *};

use chrono::{prelude::*, Duration};
use ulid::Ulid;

use std::collections::BTreeMap;

type ClientId = Ulid;
type TimeBlockId = Ulid;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        clients: RemoteData::NotAsked,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
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

pub struct Client {
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

pub enum TimeBlockStatus {
    NonBillable,
    Unpaid,
    Paid,
}

struct Invoice {
    custom_id: Option<String>,
    url: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
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

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ClientsFetched(Ok(clients)) => {},
        Msg::ClientsFetched(Err(fetch_error)) => {},

        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},

        Msg::ClearErrors => {},

        // ------ TimeBlock ------
        
        Msg::AddTimeBlock(client_id) => {},
        Msg::DeleteTimeBlock(client_id, time_block_id) => {},
        Msg::SetTimeBlockStatus(client_id, time_block_id, time_block_status) => {},

        Msg::TimeBlockDurationChanged(client_id, time_block_id, duration) => {},
        Msg::SaveTimeBlockDuration(client_id, time_block_id) => {},

        // ------ Invoice ------

        Msg::AttachInvoice(client_id, time_block_id) => {},
        Msg::DeleteInvoice(client_id, time_block_id) => {},

        Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id) => {},
        Msg::SaveInvoiceCustomId(client_id, time_block_id) => {},

        Msg::InvoiceUrlChanged(client_id, time_block_id, url) => {},
        Msg::SaveInvoiceUrl(client_id, time_block_id) => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["TimeBlocks view"]
}
```

</details>

---

## src/page/settings.rs

<details>
<summary>Code</summary>

```rust
use seed::{prelude::*, *};
use chrono::prelude::*;

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        changes_status: ChangesStatus::NoChanges,
        errors: Vec::new(),

        form: Form {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),

            errors: FormErrors::default(),
        }
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
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

#[derive(Default)]
struct FormErrors {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
    confirm_password: Option<String>,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ChangesSaved(Option<FetchError>),
    ClearErrors,

    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    ConfirmPasswordChanged(String),

    Save,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangesSaved(None) => {},
        Msg::ChangesSaved(Some(fetch_error)) => {},
        Msg::ClearErrors => {},

        Msg::UsernameChanged(username) => {},
        Msg::EmailChanged(email) => {},
        Msg::PasswordChanged(password) => {},
        Msg::ConfirmPasswordChanged(confirm_password) => {},

        Msg::Save => {},
        Msg::DeleteAccount => {},
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div!["Settings view"]
}
```

</details>

_Note:_ `Default` isn't derived for `Form` because we'll init some its fields from the "global" `User` / `Context`.

---

## src/page/not_found.rs

```rust
use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
    div!["NotFound view"]
}
```

---

Try to open your running project in a browser - [localhost:8000/](http://localhost:8000/) - and you should see a blank window with the text "Root view" in the top-left corner.

We'll write a header and learn how to switch between pages in the next chapter.

