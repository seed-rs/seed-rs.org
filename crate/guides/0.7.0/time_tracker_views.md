# Views

"Real" data are loaded in `Model`s for pages `clients_and_projects`, `time_tracker` and `time_blocks` from the backend. So let's write `view` functions for these pages to display them. 

We'll also write `view` for the `Home` page and improve our header style a bit to match  the look of pages.

We won't write `view` for `Settings` page - let's focus on it later because it's quite different from the other pages. And we won't write `view` functions for errors and saving changes in this chapter.

## Home

![Views - Home](/static/images/time_tracker_views_home.png)

1. Let's improve the header first in `lib.rs.`

    A blue background:
    ```rust
    fn view_navbar(...) -> Node<Msg> {
        nav![
            C!["navbar", "is-link"],
            ...
        ]
    }
    ```

    Fix the hamburger position:
    ```rust
    fn view_brand_and_hamburger(...) -> Node<Msg> {
        div![
            ...
            // ------ Hamburger ------
            a![
                C![...],
                style!{
                    St::MarginTop => "auto",
                    St::MarginBottom => "auto",
                },
                attrs!{
                    ...
    }
    ```

    Remove the menu item underline and other extra styles by removing `is-tab` class:
    ```rust
    fn view_navbar_menu_start(base_url: &Url, page: &Page) -> Node<Msg> {
        div![
            ...
            a![
                C!["navbar-item", IF!(matches!(page, Page::TimeTracker(_)) => "is-active"),],
                ...
            ],
            a![
                C!["navbar-item", IF!(matches!(page, Page::ClientsAndProjects(_)) => "is-active"),],
                ...
            ],
            a![
                C!["navbar-item", IF!(matches!(page, Page::TimeBlocks(_)) => "is-active"),],
                ..
            
    ```

1. We need to pass `base_url` to `page::home::view` in `lib.rs` because the `Go to Time Tracker` button will use a typed url as the link.

    ```rust
    fn view(model: &Model) -> Vec<Node<Msg>> {
        vec![
            ...
            view_content(&model.page, &model.base_url),
        ]
    }

    // ----- view_content ------

    fn view_content(page: &Page, base_url: &Url) -> Node<Msg> {
        div![
            C!["container"],
            match page {
                Page::Home => page::home::view(base_url),
                ...
            }
        ]
    }
    ```

1. And finally the `view` function in `src/page/home.rs`.

    ```rust
    use seed::{prelude::*, *};
    use crate::Urls;

    pub fn view<Ms>(base_url: &Url) -> Node<Ms> {
        section![C!["hero", "is-medium", "ml-6"],
            div![C!["hero-body"],
                h1![C!["title", "is-size-1"],
                    "Time Tracker",
                ],
                a![attrs!{At::Href => "https://seed-rs.org/"},
                    h2![C!["subtitle", "is-size-3"],
                        "seed-rs.org"
                    ]
                ],
                a![C!["button", "is-primary", "mt-5", "is-size-5"], attrs!{At::Href => Urls::new(base_url).time_tracker()},
                    strong!["Go to Time Tracker"],
                ],
            ]
        ]
    }
    ```

_Note:_ `view` functions and their helpers are often pretty long because they contain a lot of HTML-like objects. However there shouldn't be any complex logic - [Bulma's docs](https://bulma.io/documentation/) and [MDN CSS docs](https://developer.mozilla.org/en-US/docs/Web/CSS) should be enough to help you to understand them.

## Clients & Projects

![Views - Clients & Projects](/static/images/time_tracker_views_clients_and_projects.png)

1. We need to include [Font Awesome](https://fontawesome.com/) to `index.html` to have nice icons for add and delete buttons:

    ```html
    <head> 
        ...
        <script src="https://kit.fontawesome.com/e241fbfccc.js" crossorigin="anonymous"></script> 
    </head> 
    ```

    _Note:_ The link was automatically generated for me by the FA website and there aren't any special configurations. I believe you can use a common FA script hosted on a CDN or integrate it directly into the project as an alternative.

1. Let's open `src/page/clients_and_projects.rs`.

1. We need to store the primary color because we'll need in later in custom CSS styles in `view`.

    ```rust
    use crate::graphql;

    const PRIMARY_COLOR: &str = "#00d1b2";

    type ClientId = Ulid;
    ```

1. The `RemoteData` method `loaded_mut` will help us make the code cleaner in the `update` function. Let's add also code section dividers and reorder items in the `Model` block a bit to improve readability.

    ```rust
    pub struct Model {
        ...
    }

    enum ChangesStatus {
        ...
    }

    // ---- Remote Data ----

    enum RemoteData<T> {
        NotAsked,
        Loading,
        Loaded(T),
    }

    impl<T> RemoteData<T> {
        fn loaded_mut(&mut self) -> Option<&mut T> {
            if let Self::Loaded(data) = self {
                Some(data)
            } else {
                None
            }
        }
    }

    // --- Entities ----

    #[derive(Debug)]
    pub struct Client {
        ...
    }

    #[derive(Debug)]
    struct Project {
        ...
    }
    ```

    _Note:_ The general rule is to not implement anything for `Model` items. We've broken the rule by implementing `loaded_mut` for `RemoteData`. However `RemoteData` isn't actually a part of our business logic and it will be probably refactored out in the future to a general container.

1. Let's write a partial `update` function implementation. It'll help us to write and debug the `view` function.

    <details>
    <summary><code>update</code> function</summary>

    ```rust
    pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },

            Msg::ChangesSaved(None) => {
                log!("Msg::ChangesSaved");
            },
            Msg::ChangesSaved(Some(fetch_error)) => {
                log!("Msg::ChangesSaved", fetch_error);
            },

            Msg::ClearErrors => {
                log!("Msg::ClearErrors");
            },

            // ------ Client ------

            Msg::AddClient => {
                log!("Msg::AddClient");
            },
            Msg::DeleteClient(client_id) => {
                log!("Msg::DeleteClient", client_id);
            },

            Msg::ClientNameChanged(client_id, name) => {
                let mut set_client_name = move |name| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .name = name)
                };
                log!("Msg::ClientNameChanged", client_id, name);
                set_client_name(name);
            },
            Msg::SaveClientName(client_id) => {
                log!("Msg::SaveClientName", client_id);
            },

            // ------ Project ------

            Msg::AddProject(client_id) => {
                log!("Msg::AddProject", client_id);
            },
            Msg::DeleteProject(client_id, project_id) => {
                log!("Msg::DeleteProject", client_id, project_id);
            },

            Msg::ProjectNameChanged(client_id, project_id, name) => {
                let mut set_project_name = move |name| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .name = name)
                };
                log!("Msg::ProjectNameChanged", client_id, project_id, name);
                set_project_name(name);
            },
            Msg::SaveProjectName(client_id, project_id) => {
                log!("Msg::SaveProjectName", client_id, project_id);
            },
        }
    }
    ```

    </details>

1. And finally the `view` function:

    <details>
    <summary><code>view</code> function</code></summary>
    
    ```rust
    pub fn view(model: &Model) -> Node<Msg> {
        section![
            h1![C!["title", "ml-6", "my-6"],
                "Clients & Projects",
            ],
            div![C!["columns", "is-centered"],
                div![C!["column", "is-half"],
                    view_add_client_button(),
                    match &model.clients {
                        RemoteData::NotAsked | RemoteData::Loading => {
                            progress![C!["progress", "is-link", "mt-6"]].into_nodes()
                        },
                        RemoteData::Loaded(clients) => {
                            clients.iter().rev().map(|(client_id, client)| view_client(*client_id, client)).collect()
                        }
                    }
                ]
            ]
        ]
    }

    fn view_add_client_button() -> Node<Msg> {
        div![C!["level", "is-mobile"],
            button![C!["button", "is-primary", "is-rounded"],
                style!{
                    St::MarginLeft => "auto",
                    St::MarginRight => "auto",
                },
                ev(Ev::Click, |_| Msg::AddClient),
                span![C!["icon"],
                    i![C!["fas", "fa-plus"]]
                ],
                span!["Add Client"],
            ],
        ]
    }

    fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
        div![C!["box", "has-background-link", "mt-6"],
            div![C!["level", "is-mobile"],
                input![C!["input", "is-size-3", "has-text-link-light"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Height => rem(3.5),
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => percent(85),
                    },
                    attrs!{At::Value => client.name},
                    input_ev(Ev::Input, move |name| Msg::ClientNameChanged(client_id, name)),
                    ev(Ev::Change, move |_| Msg::SaveClientName(client_id)),
                ],
                view_delete_button(move || Msg::DeleteClient(client_id)),
            ],
            view_add_project_button(client_id),
            client.projects.iter().rev().map(|(project_id, project)| view_project(client_id, *project_id, project)),
        ]
    }

    fn view_add_project_button(client_id: ClientId) -> Node<Msg> {
        div![C!["level", "is-mobile"],
            button![C!["button", "is-primary", "is-rounded"],
                style!{
                    St::MarginLeft => "auto",
                    St::MarginRight => "auto",
                },
                ev(Ev::Click, move |_| Msg::AddProject(client_id)),
                span![C!["icon"],
                    i![C!["fas", "fa-plus"]]
                ],
                span!["Add Project"],
            ],
        ]
    }

    fn view_project(client_id: ClientId, project_id: ProjectId, project: &Project) -> Node<Msg> {
        div![C!["box"],
            div![C!["level", "is-mobile"],
                input![C!["input", "is-size-4"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Height => rem(3),
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => percent(85),
                    },
                    attrs!{At::Value => project.name},
                    input_ev(Ev::Input, move |name| Msg::ProjectNameChanged(client_id, project_id, name)),
                    ev(Ev::Change, move |_| Msg::SaveProjectName(client_id, project_id)),
                ],
                view_delete_button(move || Msg::DeleteProject(client_id, project_id)),
            ],
        ]
    }

    fn view_delete_button(on_click: impl Fn() -> Msg + Clone + 'static) -> Node<Msg> {
        button![C!["button", "is-primary", "is-rounded"],
            style!{
                St::Width => 0,
            },
            ev(Ev::Click, move |_| on_click()),
            span![C!["icon"],
                i![C!["fas", "fa-trash-alt"]]
            ],
        ]
    }
    ```
    
    </details>

## Time Tracker

![Views - Time Tracker](/static/images/time_tracker_views_time_tracker.png)

1. Let's open `page/time_tracker.rs`.

1. We need to store two colors in this case:

    ```rust
    use crate::graphql;

    const PRIMARY_COLOR: &str = "#00d1b2";
    const LINK_COLOR: &str = "#3273dc";

    type ClientId = Ulid;
    ```

1. Add the method `loaded_mut` just like in the previous page:

    ```rust
    enum ChangesStatus {
        NoChanges,
        Saving { requests_in_flight: usize },
        Saved(DateTime<Local>),
    }

    // ---- Remote Data ----

    enum RemoteData<T> {
        NotAsked,
        Loading,
        Loaded(T),
    }

    impl<T> RemoteData<T> {
        fn loaded_mut(&mut self) -> Option<&mut T> {
            if let Self::Loaded(data) = self {
                Some(data)
            } else {
                None
            }
        }
    }

    // --- Entities ----
    ```

1. We need to change `Model` a bit because we forgot to take into account one thing - we can't save values from text fields for `TimeEntry` `started`, `stopped` and `duration` directly to our `Model` because they may contain invalid values. Let's keep their values in a separate property `change` until they are validated and saved into the `Model`.

    Also it's more practical to set `started` and `stopped` date and time independently and save them at once together with `duration`. It means we also need to update `Msg`. 

    ```rust
    async fn request_clients() -> ... {
        ...
        let time_entry_mapper = |time_entry: query_mod::TimeEntry| (
            time_entry.id.parse().expect("parse time_entry Ulid"),
            TimeEntry {
                ...
                change: None,
            }
        );

    ...

    #[derive(Debug)]
    struct TimeEntry {
        ...
        change: Option<TimeEntryChange>,
    }

    #[derive(Debug)]
    enum TimeEntryChange {
        StartedDate(String),
        StartedTime(String),
        StoppedDate(String),
        StoppedTime(String),
        Duration(String),
    }

    // ------ ------
    //    Update
    // ------ ------

    pub enum Msg {
        ...
        
        TimeEntryNameChanged(ClientId, ProjectId, TimeEntryId, String),
        SaveTimeEntryName(ClientId, ProjectId, TimeEntryId),
        
        TimeEntryStartedDateChanged(ClientId, ProjectId, TimeEntryId, String),
        TimeEntryStartedTimeChanged(ClientId, ProjectId, TimeEntryId, String),

        TimeEntryDurationChanged(ClientId, ProjectId, TimeEntryId, String),
        
        TimeEntryStoppedDateChanged(ClientId, ProjectId, TimeEntryId, String),
        TimeEntryStoppedTimeChanged(ClientId, ProjectId, TimeEntryId, String),

        SaveTimeEntryChange(ClientId, ProjectId, TimeEntryId),

        OnSecondTick,
    }
    ```

1. Our `update` function is longer than the one in the previous page and contains some boilerplate. I don't recommend to refactor it in this phase, let's wait until all `match` arms are implemented and the code is stabilized so we can see all patterns clearly.

    <details>
    <summary><code>update</code> function</summary>

    ```rust
    pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },

            Msg::ChangesSaved(None) => {
                log!("Msg::ChangesSaved");
            },
            Msg::ChangesSaved(Some(fetch_error)) => {
                log!("Msg::ChangesSaved", fetch_error);
            },

            Msg::ClearErrors => {
                log!("Msg::ClearErrors");
            },

            Msg::Start(client_id, project_id) => {
                log!("Msg::Start", client_id, project_id);
            },
            Msg::Stop(client_id, project_id) => {
                log!("Msg::Stop", client_id, project_id);
            },

            Msg::DeleteTimeEntry(client_id, project_id, time_entry_id) => {
                log!("Msg::DeleteTimeEntry", client_id, project_id, time_entry_id);
            },

            Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name) => {
                let mut set_time_entry_name = move |name| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .name = name)
                };
                log!("Msg::TimeEntryNameChanged", client_id, project_id, time_entry_id, name);
                set_time_entry_name(name);
            },
            Msg::SaveTimeEntryName(client_id, project_id, time_entry_id) => {
                log!("Msg::SaveTimeEntryName", client_id, project_id, time_entry_id);
            },

            Msg::TimeEntryStartedDateChanged(client_id, project_id, time_entry_id, date) => {
                let mut set_time_entry_change = move |change| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = Some(change))
                };
                log!("Msg::TimeEntryStartedDateChanged", client_id, project_id, time_entry_id, date);
                set_time_entry_change(TimeEntryChange::StartedDate(date));
            },
            Msg::TimeEntryStartedTimeChanged(client_id, project_id, time_entry_id, time) => {
                let mut set_time_entry_change = move |change| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = Some(change))
                };
                log!("Msg::TimeEntryStartedTimeChanged", client_id, project_id, time_entry_id, time);
                set_time_entry_change(TimeEntryChange::StartedTime(time));
            },

            Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, duration) => {
                let mut set_time_entry_change = move |change| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = Some(change))
                };
                log!("Msg::TimeEntryDurationChanged", client_id, project_id, time_entry_id, duration);
                set_time_entry_change(TimeEntryChange::Duration(duration));
            },

            Msg::TimeEntryStoppedDateChanged(client_id, project_id, time_entry_id, date) => {
                let mut set_time_entry_change = move |change| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = Some(change))
                };
                log!("Msg::TimeEntryStoppedDateChanged", client_id, project_id, time_entry_id, date);
                set_time_entry_change(TimeEntryChange::StoppedDate(date));
            },
            Msg::TimeEntryStoppedTimeChanged(client_id, project_id, time_entry_id, time) => {
                let mut set_time_entry_change = move |change| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = Some(change))
                };
                log!("Msg::TimeEntryStoppedTimeChanged", client_id, project_id, time_entry_id, time);
                set_time_entry_change(TimeEntryChange::StoppedTime(time));
            },

            Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id) => {
                let mut delete_time_entry_change = move || -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?
                        .change = None)
                };
                log!("Msg::SaveTimeEntryChange", client_id, project_id, time_entry_id);
                delete_time_entry_change();
            },

            Msg::OnSecondTick => (),
        }
    }
    ```

    </details>

1. And finally the `view`:

    <details>
    <summary><code>view</code> function</summary>

    ```rust
    pub fn view(model: &Model) -> Node<Msg> {
        section![
            h1![C!["title", "ml-6", "mt-6", "mb-5"],
                "Time Tracker",
            ],
            div![C!["columns", "is-centered"],
                div![C!["column", "is-two-thirds"],
                    match &model.clients {
                        RemoteData::NotAsked | RemoteData::Loading => {
                            progress![C!["progress", "is-link", "mt-6"]].into_nodes()
                        },
                        RemoteData::Loaded(clients) => {
                            clients.iter().rev().map(|(client_id, client)| view_client(*client_id, client)).collect()
                        }
                    }
                ]
            ]
        ]
    }

    fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
        div![C!["box", "has-background-link", "mt-6",],
            div![C!["level", "is-mobile"],
                div![C!["is-size-3", "has-text-link-light"], 
                    &client.name,
                ],
            ],
            client.projects.iter().rev().map(|(project_id, project)| view_project(client_id, *project_id, project)),
        ]
    }

    fn view_project(client_id: ClientId, project_id: ProjectId, project: &Project) -> Node<Msg> {
        let active_time_entry = project
            .time_entries
            .iter()
            .find(|(_, time_entry)| time_entry.stopped.is_none());

        div![C!["box", "mt-6"],
            div![C!["level", "is-mobile"],
                div![C!["is-size-4"], 
                    &project.name,
                ],
                view_start_stop_button(client_id, project_id, active_time_entry.is_some()),
            ],
            project.time_entries.iter().rev().map(|(time_entry_id, time_entry)| {
                view_time_entry(client_id, project_id, *time_entry_id, time_entry)
            }),
        ]
    }

    fn view_start_stop_button(client_id: ClientId, project_id: ProjectId, started: bool) -> Node<Msg> {
        div![C!["level", "is-mobile"],
            button![C!["button", if started { "is-warning" } else { "is-primary" }, "is-rounded"],
                ev(Ev::Click, move |_| if started { 
                    Msg::Stop(client_id, project_id) 
                } else { 
                    Msg::Start(client_id, project_id) 
                }),
                span![if started { "Stop" } else { "Start" }],
            ],
        ]
    }

    fn view_time_entry(
        client_id: ClientId, 
        project_id: ProjectId, 
        time_entry_id: TimeEntryId, 
        time_entry: &TimeEntry
    ) -> Node<Msg> {
        let active = time_entry.stopped.is_none();
        let stopped = time_entry.stopped.as_ref().cloned().unwrap_or_else(chrono::Local::now);
        let duration = stopped - time_entry.started;

        div![C!["box", if active { "has-background-warning" } else { "has-background-link"}, IF!(not(active) => "has-text-link-light")],
            div![C!["level", "is-mobile"], style!{St::MarginBottom => px(5)},
                input![C!["input", "is-size-4", IF!(not(active) => "has-text-link-light")], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Height => rem(3),
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", if active { LINK_COLOR } else { PRIMARY_COLOR }, px(2)),
                        St::MaxWidth => percent(85),
                    },
                    attrs!{At::Value => time_entry.name},
                    input_ev(Ev::Input, move |name| Msg::TimeEntryNameChanged(client_id, project_id, time_entry_id, name)),
                    ev(Ev::Change, move |_| Msg::SaveTimeEntryName(client_id, project_id, time_entry_id)),
                ],
                view_delete_button(move || Msg::DeleteTimeEntry(client_id, project_id, time_entry_id), active),
            ],
            div![C!["level", "is-mobile", "is-hidden-tablet"], style!{St::MarginBottom => 0},
                view_duration(client_id, project_id, time_entry_id, &duration, time_entry.change.as_ref(), active)
            ],
            div![C!["level", "is-mobile"],
                view_started(client_id, project_id, time_entry_id, time_entry.change.as_ref(), active, &time_entry.started),
                div![C!["is-hidden-mobile"],
                    view_duration(client_id, project_id, time_entry_id, &duration, time_entry.change.as_ref(), active),
                ],
                view_stopped(client_id, project_id, time_entry_id,  time_entry.change.as_ref(), active, &stopped),
            ],
        ]
    }

    fn view_started(
        client_id: ClientId, 
        project_id: ProjectId, 
        time_entry_id: TimeEntryId, 
        time_entry_change: Option<&TimeEntryChange>,
        for_active_time_entry: bool,
        started: &chrono::DateTime<chrono::Local>,
    ) -> Node<Msg> {
        div![C!["is-flex"], style!{St::FlexDirection => "column"},
            input![C!["input", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }],
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(2),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(1)),
                    St::MaxWidth => rem(10),
                },
                attrs!{
                    At::Value => if let Some(TimeEntryChange::StartedDate(date)) = time_entry_change {
                        date.to_owned()
                    } else {
                        started.format("%F").to_string()
                    }
                },
                input_ev(Ev::Input, move |date| Msg::TimeEntryStartedDateChanged(client_id, project_id, time_entry_id, date)),
                ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
            ],
            input![C!["input", "is-size-5", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3),
                    St::Border => "none",
                    St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                    St::MaxWidth => rem(10),
                },
                attrs!{
                    At::Value => if let Some(TimeEntryChange::StartedTime(time)) = time_entry_change {
                        time.to_owned()
                    } else {
                        started.format("%X").to_string()
                    }
                },
                input_ev(Ev::Input, move |time| Msg::TimeEntryStartedTimeChanged(client_id, project_id, time_entry_id, time)),
                ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
            ],
        ]
    }

    fn view_stopped(
        client_id: ClientId, 
        project_id: ProjectId, 
        time_entry_id: TimeEntryId, 
        time_entry_change: Option<&TimeEntryChange>,
        for_active_time_entry: bool,
        stopped: &chrono::DateTime<chrono::Local>,
    ) -> Node<Msg> {
        div![C!["is-flex"], style!{St::FlexDirection => "column"},
            input![C!["input", "has-text-centered", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }],
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(2),
                    St::Border => "none",
                    St::BorderBottom => IF!(not(for_active_time_entry) => {
                        format!("{} {} {}", "solid", PRIMARY_COLOR, px(1))
                    }),
                    St::MaxWidth => rem(10),
                },
                attrs!{
                    At::Disabled => for_active_time_entry.as_at_value(),
                    At::Value => if let Some(TimeEntryChange::StoppedDate(date)) = time_entry_change {
                        date.to_owned()
                    } else {
                        stopped.format("%F").to_string()
                    }
                },
                input_ev(Ev::Input, move |date| Msg::TimeEntryStoppedDateChanged(client_id, project_id, time_entry_id, date)),
                ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
            ],
            input![C!["input", "has-text-centered", "is-size-5", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
                style!{
                    St::BoxShadow => "none",
                    St::BackgroundColor => "transparent",
                    St::Height => rem(3),
                    St::Border => "none",
                    St::BorderBottom => IF!(not(for_active_time_entry) => {
                        format!("{} {} {}", "solid", PRIMARY_COLOR, px(2))
                    }),
                    St::MaxWidth => rem(10),
                },
                attrs!{
                    At::Disabled => for_active_time_entry.as_at_value(),
                    At::Value => if let Some(TimeEntryChange::StoppedTime(time)) = time_entry_change {
                        time.to_owned()
                    } else {
                        stopped.format("%X").to_string()
                    }
                },
                input_ev(Ev::Input, move |time| Msg::TimeEntryStoppedTimeChanged(client_id, project_id, time_entry_id, time)),
                ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
            ],
        ]
    }

    fn view_duration(
        client_id: ClientId, 
        project_id: ProjectId, 
        time_entry_id: TimeEntryId, 
        duration: &chrono::Duration, 
        time_entry_change: Option<&TimeEntryChange>, 
        for_active_time_entry: bool
    ) -> Node<Msg> {
        let num_seconds = duration.num_seconds();
        let hours = num_seconds / 3600;
        let minutes = num_seconds % 3600 / 60;
        let seconds = num_seconds % 60;

        input![C!["input", "has-text-centered", "is-size-4", if for_active_time_entry { "has-text-dark" } else { "has-text-link-light" }], 
            style!{
                St::Margin => "auto",
                St::BoxShadow => "none",
                St::BackgroundColor => "transparent",
                St::Height => rem(3),
                St::Border => "none",
                St::BorderBottom => IF!(not(for_active_time_entry) => {
                    format!("{} {} {}", "solid", PRIMARY_COLOR, px(2))
                }),
                St::MaxWidth => rem(10),
            },
            attrs!{
                At::Disabled => for_active_time_entry.as_at_value(),
                At::Value => if let Some(TimeEntryChange::Duration(duration)) = time_entry_change {
                    duration.to_owned()
                } else {
                    format!("{}:{:02}:{:02}", hours, minutes, seconds)
                }
            },
            input_ev(Ev::Input, move |duration| Msg::TimeEntryDurationChanged(client_id, project_id, time_entry_id, duration)),
            ev(Ev::Change, move |_| Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id)),
        ]
    }

    fn view_delete_button(on_click: impl Fn() -> Msg + Clone + 'static, for_active_time_entry: bool) -> Node<Msg> {
        button![C!["button", if for_active_time_entry { "is-link" } else { "is-primary" }, "is-rounded"],
            style!{
                St::Width => 0,
            },
            ev(Ev::Click, move |_| on_click()),
            span![C!["icon"],
                i![C!["fas", "fa-trash-alt"]]
            ],
        ]
    }
    ```

    </details>

## Time Blocks

![Views - Time Blocks](/static/images/time_tracker_views_time_blocks.png)

1. Let's open `src/page/time_block.rs`.

1. One primary color:

    ```rust
    use crate::graphql;

    const PRIMARY_COLOR: &str = "#00d1b2";

    type ClientId = Ulid;
    ```

1. The method `loaded_mut`:

    ```rust
    enum ChangesStatus {
        NoChanges,
        Saving { requests_in_flight: usize },
        Saved(DateTime<Local>),
    }

    // ---- Remote Data ----

    enum RemoteData<T> {
        NotAsked,
        Loading,
        Loaded(T),
    }

    impl<T> RemoteData<T> {
        fn loaded_mut(&mut self) -> Option<&mut T> {
            if let Self::Loaded(data) = self {
                Some(data)
            } else {
                None
            }
        }
    }

    // --- Entities ----
    ```

1. We should make `TimeBlockStatus` copyable:

    ```rust
    #[derive(Debug, Copy, Clone)]
    pub enum TimeBlockStatus {
        ...
    }
    ```

1. We have to resolve the similar problem like in the previous page - we can't save `duration` directly, we have to validate it first.

    ```rust
    async fn request_clients() -> ... {
        ...

        let time_block_mapper = |time_block: query_mod::TimeBlock| (
            time_block.id.parse().expect("parse time_block Ulid"), 
            TimeBlock { 
                ...
                duration: Duration::seconds(i64::from(time_block.duration)),
                duration_change: None,
                invoice: time_block.invoice.map(invoice_mapper),
            }
        );

    ...

    #[derive(Debug)]
    struct TimeBlock {
        ...
        duration: Duration,
        duration_change: Option<String>,
        invoice: Option<Invoice>,
    }

    ```

1. Let's do one business logic change. I think we should take into account also currently running `TimeEntry` while we are calculating the total `tracked` time.

    ```rust
    async fn request_clients() -> graphql::Result<BTreeMap<ClientId, Client>> {
        ...

        let compute_tracked_time = |projects: Vec<query_mod::Project>| {
            projects
                .into_iter()
                .flat_map(|project| project.time_entries)
                .map(|time_entry| {
                    let started: DateTime<Local> = 
                        time_entry.started.0.parse().expect("parse time_entry started");
                    
                    let stopped: DateTime<Local> = if let Some(stopped) = time_entry.stopped {
                        stopped.0.parse().expect("parse time_entry stopped")
                    } else {
                        chrono::Local::now()
                    };
                    
                    stopped - started
                })
                .fold(Duration::seconds(0), Duration::add)
        };
    ```

1. Our updated `update` function:

<details>
<summary><code>update</code> function</summary>

    ```rust
    pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
                log!("Msg::ClientsFetched", clients);
                model.clients = RemoteData::Loaded(clients);
            },
            Msg::ClientsFetched(Err(graphql_error)) => {
                model.errors.push(graphql_error);
            },

            Msg::ChangesSaved(None) => {
                log!("Msg::ChangesSaved");
            },
            Msg::ChangesSaved(Some(fetch_error)) => {
                log!("Msg::ChangesSaved", fetch_error);
            },

            Msg::ClearErrors => {},

            // ------ TimeBlock ------
            
            Msg::AddTimeBlock(client_id) => {
                log!("Msg::AddTimeBlock", client_id);
            },
            Msg::DeleteTimeBlock(client_id, time_block_id) => {
                log!("Msg::DeleteTimeBlock", client_id, time_block_id);
            },
            Msg::SetTimeBlockStatus(client_id, time_block_id, time_block_status) => {
                log!("Msg::SetTimeBlockStatus", client_id, time_block_id, time_block_status);
            },

            Msg::TimeBlockNameChanged(client_id, time_block_id, name) => {
                let mut set_time_block_name = move |name| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .name = name)
                };
                log!("Msg::TimeBlockNameChanged", client_id, time_block_id, name);
                set_time_block_name(name);
            },
            Msg::SaveTimeBlockName(client_id, time_block_id) => {
                log!("Msg::SaveTimeBlockName", client_id, time_block_id);
            },

            Msg::TimeBlockDurationChanged(client_id, time_block_id, duration) => {
                let mut set_time_block_duration_change = move |duration| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .duration_change = Some(duration))
                };
                log!("Msg::TimeBlockDurationChanged", client_id, time_block_id, duration);
                set_time_block_duration_change(duration);
            },
            Msg::SaveTimeBlockDuration(client_id, time_block_id) => {
                let mut set_time_block_duration_change = move || -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .duration_change = None)
                };
                log!("Msg::SaveTimeBlockDuration", client_id, time_block_id);
                set_time_block_duration_change();
            },

            // ------ Invoice ------

            Msg::AttachInvoice(client_id, time_block_id) => {
                log!("Msg::AttachInvoice", client_id, time_block_id);
            },
            Msg::DeleteInvoice(client_id, time_block_id) => {
                log!("Msg::DeleteInvoice", client_id, time_block_id);
            },

            Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id) => {
                let mut set_invoice_custom_id = move |custom_id| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .invoice.as_mut()?
                        .custom_id = Some(custom_id))
                };
                log!("Msg::InvoiceCustomIdChanged", client_id, time_block_id, custom_id);
                set_invoice_custom_id(custom_id);
                
            },
            Msg::SaveInvoiceCustomId(client_id, time_block_id) => {
                log!("Msg::SaveInvoiceCustomId", client_id, time_block_id);
            },

            Msg::InvoiceUrlChanged(client_id, time_block_id, url) => {
                let mut set_invoice_url = move |url| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .invoice.as_mut()?
                        .url = Some(url))
                };
                log!("Msg::InvoiceUrlChanged", client_id, time_block_id, url);
                set_invoice_url(url);
            },
            Msg::SaveInvoiceUrl(client_id, time_block_id) => {
                log!("Msg::SaveInvoiceUrl", client_id, time_block_id);
            },
        }
    }
    ```

</details>

1. And finally the `view` function:

    <details>
    <summary><code>view</code> function</summary>

    ```rust
    pub fn view(model: &Model) -> Node<Msg> {
        section![
            h1![C!["title", "ml-6", "mt-6", "mb-5"],
                "Time Blocks",
            ],
            div![C!["columns", "is-centered"],
                div![C!["column", "is-two-thirds"],
                    match &model.clients {
                        RemoteData::NotAsked | RemoteData::Loading => {
                            progress![C!["progress", "is-link", "mt-6"]].into_nodes()
                        },
                        RemoteData::Loaded(clients) => {
                            clients.iter().rev().map(|(client_id, client)| view_client(*client_id, client)).collect()
                        }
                    }
                ]
            ]
        ]
    }

    fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
        div![C!["box", "has-background-link", "mt-6",],
            div![C!["level", "is-mobile"], style!{St::FlexWrap => "wrap", St::MarginBottom => 0},
                div![C!["is-size-3", "has-text-link-light", "mb-2"], 
                    &client.name,
                ],
                view_statistics(client.time_blocks.values(), &client.tracked),
            ],
            view_add_time_block_button(client_id),
            client.time_blocks.iter().rev().map(|(time_block_id, time_block)| view_time_block(client_id, *time_block_id, time_block)),
        ]
    }

    fn view_statistics<'a>(time_blocks: impl Iterator<Item = &'a TimeBlock>, tracked: &Duration, ) -> Node<Msg> {
        let mut blocked = 0.;
        let mut unpaid_total = 0.;
        let mut paid_total = 0.;

        for time_block in time_blocks {
            let hours = time_block.duration.num_minutes() as f64 / 60.;
            blocked += hours;

            match time_block.status {
                TimeBlockStatus::NonBillable => (),
                TimeBlockStatus::Unpaid => unpaid_total += hours,
                TimeBlockStatus::Paid => paid_total += hours,
            };
        }

        let tracked = tracked.num_minutes() as f64 / 60.;
        let to_block = tracked - blocked;

        let pair = |key: &str, value: f64| {
            div![C!["is-flex"], style!{St::JustifyContent => "space-between"},
                span![
                    key
                ],
                span![style!{St::MarginLeft => rem(1)},
                    format!("{:.1}", value)
                ],
            ]
        };

        div![C!["level", "is-mobile"], style!{St::AlignItems => "baseline"},
            div![C!["box", "has-background-link", "has-text-link-light"],
                pair("Blocked", blocked),
                div![style!{St::Height => rem(1)}],
                pair("Unpaid", unpaid_total),
                pair("Paid", paid_total),
            ],
            div![
                div![C!["box", "has-background-link", "has-text-link-light"],
                    style!{St::MarginBottom => 0},
                    pair("Tracked", tracked),
                ],
                div![C!["box", "has-background-link", "has-text-link-light"],
                    pair("To Block", to_block),
                ],
            ]
        ]
    }

    fn view_add_time_block_button(client_id: ClientId) -> Node<Msg> {
        div![C!["level", "is-mobile"],
            button![C!["button", "is-primary", "is-rounded"],
                style!{
                    St::MarginLeft => "auto",
                    St::MarginRight => "auto",
                },
                ev(Ev::Click, move |_| Msg::AddTimeBlock(client_id)),
                span![C!["icon"],
                    i![C!["fas", "fa-plus"]]
                ],
                span!["Add Time Block"],
            ],
        ]
    }

    fn view_time_block(client_id: ClientId, time_block_id: TimeBlockId, time_block: &TimeBlock) -> Node<Msg> {
        div![C!["box"],
            div![C!["level", "is-mobile"],
                input![C!["input", "is-size-4"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Height => rem(3),
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => percent(47),
                    },
                    attrs!{At::Value => time_block.name},
                    input_ev(Ev::Input, move |name| Msg::TimeBlockNameChanged(client_id, time_block_id, name)),
                    ev(Ev::Change, move |_| Msg::SaveTimeBlockName(client_id, time_block_id)),
                ],
                div![C!["is-flex"], style!{St::AlignItems => "center"},
                    input![C!["input", "is-size-4", "has-text-right"], 
                        style!{
                            St::BoxShadow => "none",
                            St::BackgroundColor => "transparent",
                            St::Height => rem(3),
                            St::Border => "none",
                            St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                            St::MaxWidth => rem(6),
                        },
                        attrs!{
                            At::Value => if let Some(duration) = &time_block.duration_change {
                                duration.to_owned()
                            } else {
                                format!("{:.1}", time_block.duration.num_minutes() as f64 / 60.)
                            }
                        },
                        input_ev(Ev::Input, move |duration| Msg::TimeBlockDurationChanged(client_id, time_block_id, duration)),
                        ev(Ev::Change, move |_| Msg::SaveTimeBlockDuration(client_id, time_block_id)),
                    ],
                    div![
                        "h"
                    ],
                ],
                view_delete_button(move || Msg::DeleteTimeBlock(client_id, time_block_id)),
            ],
            div![C!["level", "is-mobile"],
                view_status_buttons(client_id, time_block_id, time_block.status),
                IF!(time_block.invoice.is_none() => view_attach_invoice_button(client_id, time_block_id)),
            ],
            time_block.invoice.as_ref().map(move |invoice| view_invoice(client_id, time_block_id, invoice)),
        ]
    }

    fn view_status_buttons(client_id: ClientId, time_block_id: TimeBlockId, status: TimeBlockStatus) -> Node<Msg> {
        div![C!["buttons", "has-addons"], style!{St::MarginBottom => 0},
            button![
                C!["button", "is-rounded", IF!(matches!(status, TimeBlockStatus::NonBillable) => 
                    ["is-selected", "is-primary"].as_ref()
                )], 
                style!{St::MarginBottom => 0},
                "Non-billable",
                ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::NonBillable)),
            ],
            button![
                C!["button", IF!(matches!(status, TimeBlockStatus::Unpaid) => 
                    ["is-selected", "is-primary"].as_ref()
                )], 
                style!{St::MarginBottom => 0},
                "Unpaid",
                ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::Unpaid)),
            ],
            button![
                C!["button", "is-rounded", IF!(matches!(status, TimeBlockStatus::Paid) => 
                    ["is-selected", "is-primary"].as_ref()
                )],
                style!{St::MarginBottom => 0},
                "Paid",
                ev(Ev::Click, move |_| Msg::SetTimeBlockStatus(client_id, time_block_id, TimeBlockStatus::Paid)),
            ],
        ]
    }

    fn view_attach_invoice_button(client_id: ClientId, time_block_id: TimeBlockId) -> Node<Msg> {
        button![C!["button", "is-primary", "is-rounded"],
            ev(Ev::Click, move |_| Msg::AttachInvoice(client_id, time_block_id)),
            span![C!["icon"],
                i![C!["fas", "fa-plus"]]
            ],
            span!["Attach Invoice"],
        ]
    }

    fn view_invoice(client_id: ClientId, time_block_id: TimeBlockId, invoice: &Invoice) -> Node<Msg> {
        div![C!["box", "has-text-link-light", "has-background-link"],
            div![C!["level", "is-mobile"],
                div!["Invoice ID"],
                input![C!["input", "has-text-link-light"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => percent(55),
                    },
                    attrs!{At::Value => invoice.custom_id.as_ref().map(String::as_str).unwrap_or_default()},
                    input_ev(Ev::Input, move |custom_id| Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id)),
                    ev(Ev::Change, move |_| Msg::SaveInvoiceCustomId(client_id, time_block_id)),
                ],
                view_delete_button(move || Msg::DeleteInvoice(client_id, time_block_id)),
            ],
            div![C!["level", "is-mobile"],
                div!["URL"],
                input![C!["input", "has-text-link-light"], 
                    style!{
                        St::BoxShadow => "none",
                        St::BackgroundColor => "transparent",
                        St::Border => "none",
                        St::BorderBottom => format!("{} {} {}", "solid", PRIMARY_COLOR, px(2)),
                        St::MaxWidth => percent(67),
                    },
                    attrs!{At::Value => invoice.url.as_ref().map(String::as_str).unwrap_or_default()},
                    input_ev(Ev::Input, move |url| Msg::InvoiceUrlChanged(client_id, time_block_id, url)),
                    ev(Ev::Change, move |_| Msg::SaveInvoiceUrl(client_id, time_block_id)),
                ],
                invoice.url.as_ref().map(move |url| view_go_button(url)),
            ],
        ]
    }

    fn view_delete_button(on_click: impl Fn() -> Msg + Clone + 'static) -> Node<Msg> {
        button![C!["button", "is-primary", "is-rounded"],
            style!{
                St::Width => 0,
            },
            ev(Ev::Click, move |_| on_click()),
            span![C!["icon"],
                i![C!["fas", "fa-trash-alt"]]
            ],
        ]
    }

    fn view_go_button(url: &str) -> Node<Msg> {
        a![C!["button", "is-primary", "is-rounded"],
            style!{
                St::Width => 0,
            },
            attrs!{
                At::Href => url,
                At::Target => "_blank",
            },
            span![C!["icon"],
                i![C!["fas", "fa-external-link-alt"]]
            ],
        ]
    }
    ```

    </details>

---

That was a lot of code! However we have nice and almost functional pages now.

There are some minor layout issues on the phone screen, but overall the app GUI should be responsive enough. Also I can imagine we can "compress" GUI and its items or make some items expandable or implement "list & detail pages" to improve UX when there are many items, but it would make the app more complex - it's one idea for future development.

In the next chapter, we'll implement `match` arms in `update` functions properly to make most pages almost fully functional.
