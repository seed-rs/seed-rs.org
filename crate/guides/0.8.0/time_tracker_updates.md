# Updates

Let's breathe life into our main pages! 

- We'll implement most `match` arms in our `update` functions.
- We want to focus text fields (like a `Client` name) when the corresponding entity is created.
- We'll mark the locations in code where we will be sending GraphQL requests.

We will modify our `Model`s directly without waiting for the server response because we'll try to apply _Optimistic UI_ pattern.
- It'll improve UX - the app will seem faster.
- We don't have to communicate with the server yet.
- It basically eliminates the problem when the user clicks on one button multiple times (it's a problem especially for delete buttons).
- More info about _Optimistic UI_:
    - [Optimistic UIs in under 1000 words](https://uxplanet.org/optimistic-1000-34d9eefe4c05)
    - [Optimistic UI and Clobbering](https://hasura.io/blog/optimistic-ui-and-clobbering/)

## Clients & Projects

![Time Tracker - Client & Projects update](/static/images/time_tracker_update_clients_and_projects.gif)

`src/page/clients_and_projects.rs`

1. `name_input`s in `request_clients`

    ```rust
    async fn request_clients() -> ... {
        ...
        let project_mapper = |project: query_mod::Project| (
            ... 
            Project { 
                name: project.name, 
                name_input: ElRef::new(), 
            }
        );

        let client_mapper = |client: query_mod::Client| (
            ...
            Client {
                name: client.name,
                projects: client.projects.into_iter().map(project_mapper).collect(),
                name_input: ElRef::new(),
            }
        );
    ```

1. `name_input`s in entities

    ```rust
    // --- Entities ----

    #[derive(Debug)]
    pub struct Client {
        ...
        name_input: ElRef<web_sys::HtmlInputElement>,
    }

    #[derive(Debug)]
    struct Project {
        ...
        name_input: ElRef<web_sys::HtmlInputElement>,
    }
    ```

1. `Focus***Name` in `Msg`

    ```rust
    pub enum Msg {
        ...
        DeleteClient(ClientId),
        FocusClientName(ClientId),

        ...
        DeleteProject(ClientId, ProjectId),
        FocusProjectName(ClientId, ProjectId),
        
        ...
    ```

1. `update`

    ```rust
    pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
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
                model.errors.clear();
            },

            // ------ Client ------

            Msg::AddClient => {
                if let Some(clients) = model.clients.loaded_mut() {
                    let client_id = ClientId::new();
                    let client = Client {
                        name: "".to_owned(),
                        projects: BTreeMap::new(),
                        name_input: ElRef::new(),
                    };
                    // @TODO: Send request.
                    clients.insert(client_id, client);
                    orders.after_next_render(move |_| Msg::FocusClientName(client_id));
                }
            },
            Msg::DeleteClient(client_id) => {
                let mut delete_client = move |client_id| -> Option<()> {
                    let clients = model.clients.loaded_mut()?;
                    let client_name = clients.get(&client_id).map(|client| &client.name)?;

                    if let Ok(true) = window().confirm_with_message(&format!("Client \"{}\" will be deleted.", client_name)) {
                        clients.remove(&client_id);
                        // @TODO: Send request.
                    }
                    Some(())
                };
                delete_client(client_id);
            },
            Msg::FocusClientName(client_id) => {
                let mut focus_client_name = move |client_id| -> Option<()> {
                    model
                        .clients
                        .loaded_mut()?
                        .get(&client_id)?
                        .name_input
                        .get()?
                        .focus()
                        .ok()
                };
                focus_client_name(client_id);
            }

            Msg::ClientNameChanged(client_id, name) => {
                let mut set_client_name = move |name| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .name = name)
                };
                set_client_name(name);
            },
            Msg::SaveClientName(client_id) => {
                // @TODO: Send request.
            },

            // ------ Project ------

            Msg::AddProject(client_id) => {
                let mut add_project = move |client_id| -> Option<()> {
                    let projects = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.projects;

                    let project_id = ProjectId::new();
                    let project = Project {
                        name: "".to_owned(),
                        name_input: ElRef::new(),
                    };
                    // @TODO: Send request.
                    projects.insert(project_id, project);
                    orders.after_next_render(move |_| Msg::FocusProjectName(client_id, project_id));

                    Some(())
                };
                add_project(client_id);
            },
            Msg::DeleteProject(client_id, project_id) => {
                let mut delete_project = move |client_id, project_id| -> Option<()> {
                    let projects = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.projects;
                    let project_name = projects.get(&project_id).map(|project| &project.name)?;

                    if let Ok(true) = window().confirm_with_message(&format!("Project \"{}\" will be deleted.", project_name)) {
                        projects.remove(&project_id);
                        // @TODO: Send request.
                    }
                    Some(())
                };
                delete_project(client_id, project_id);
            },
            Msg::FocusProjectName(client_id, project_id) => {
                let mut focus_project_name = move |client_id, project_id| -> Option<()> {
                    model
                        .clients
                        .loaded_mut()?
                        .get(&client_id)?
                        .projects
                        .get(&project_id)?
                        .name_input
                        .get()?
                        .focus()
                        .ok()
                };
                focus_project_name(client_id, project_id);
            }

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
                set_project_name(name);
            },
            Msg::SaveProjectName(client_id, project_id) => {
                // @TODO: Send request.
            },
        }
    }
    ```

1. `name_input`s in `view_client`

    ```rust
    fn view_client(client_id: ClientId, client: &Client) -> Node<Msg> {
        div![C!["box", ...],
            div![C!["level", ...],
                input![C!["input", ...], 
                    el_ref(&client.name_input),
                    ...
    ```

1. `name_input`s in `view_project`

    ```rust
    fn view_project(client_id: ClientId, project_id: ProjectId, project: &Project) -> Node<Msg> {
        div![C!["box"],
            div![C!["level", ...],
                input![C!["input", ...], 
                    el_ref(&project.name_input),
                    ...
    ```

## Time Blocks

![Time Tracker - Time Blocks update](/static/images/time_tracker_update_time_blocks.gif)

`src/page/time_blocks.rs`

1. `name_input` in `request_clients`

    ```rust
    async fn request_clients() -> ... {
        ...
        let time_block_mapper = |time_block: query_mod::TimeBlock| (
            ...
            TimeBlock { 
                ...
                name_input: ElRef::new(),
            }
        );
    ```

1. `name_input` in `TimeBlock`

    ```rust
    struct TimeBlock {
        ...
        name_input: ElRef<web_sys::HtmlInputElement>,
    }
    ```

1. `FocusTimeBlockName` in `Msg`

    ```rust
    pub enum Msg {
        ...
        SetTimeBlockStatus(ClientId, TimeBlockId, TimeBlockStatus),
        FocusTimeBlockName(ClientId, TimeBlockId),

        ...
    ```

1. `update`

    ```rust
    pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
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
                model.errors.clear();
            },

            // ------ TimeBlock ------
            
            Msg::AddTimeBlock(client_id) => {
                let mut add_time_block = move |client_id| -> Option<()> {
                    let time_blocks = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.time_blocks;

                    let previous_duration = time_blocks
                        .iter()
                        .next_back()
                        .map(|(_, time_block)| time_block.duration);

                    let time_block_id = TimeBlockId::new();
                    let time_block = TimeBlock {
                        name: "".to_owned(),
                        status: TimeBlockStatus::Unpaid,
                        duration: previous_duration.unwrap_or_else(|| chrono::Duration::hours(20)),
                        duration_change: None,
                        invoice: None,
                        name_input: ElRef::new(),
                    };
                    // @TODO: Send request.
                    time_blocks.insert(time_block_id, time_block);
                    orders.after_next_render(move |_| Msg::FocusTimeBlockName(client_id, time_block_id));

                    Some(())
                };
                add_time_block(client_id);
            },
            Msg::DeleteTimeBlock(client_id, time_block_id) => {
                let mut delete_time_block = move |client_id, time_block_id| -> Option<()> {
                    let time_blocks = &mut model.clients.loaded_mut()?.get_mut(&client_id)?.time_blocks;
                    let time_block_name = time_blocks.get(&time_block_id).map(|time_block| &time_block.name)?;

                    if let Ok(true) = window().confirm_with_message(&format!("Time Block \"{}\" will be deleted.", time_block_name)) {
                        time_blocks.remove(&time_block_id);
                        // @TODO: Send request.
                    }
                    Some(())
                };
                delete_time_block(client_id, time_block_id);
            },
            Msg::SetTimeBlockStatus(client_id, time_block_id, time_block_status) => {
                let mut set_time_block_status = move |status| -> Option<()> {
                    model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .status = status;
                    // @TODO: Send request.
                    Some(())
                };
                set_time_block_status(time_block_status);
            },
            Msg::FocusTimeBlockName(client_id, time_block_id) => {
                let mut focus_time_block_name = move |client_id, time_block_id| -> Option<()> {
                    model
                        .clients
                        .loaded_mut()?
                        .get(&client_id)?
                        .time_blocks
                        .get(&time_block_id)?
                        .name_input
                        .get()?
                        .focus()
                        .ok()
                };
                focus_time_block_name(client_id, time_block_id);
            }

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
                set_time_block_name(name);
            },
            Msg::SaveTimeBlockName(client_id, time_block_id) => {
                // @TODO: Send request.
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
                set_time_block_duration_change(duration);
            },
            Msg::SaveTimeBlockDuration(client_id, time_block_id) => {
                let mut set_time_block_duration = move || -> Option<()> {
                    let time_block = model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?;

                    let hours = time_block.duration_change.take()?.parse::<f64>().ok()?;
                    time_block.duration = chrono::Duration::seconds((hours * 3600.0) as i64);
                    // @TODO: Send request.
                    Some(())
                };
                set_time_block_duration();
            },

            // ------ Invoice ------

            Msg::AttachInvoice(client_id, time_block_id) => {
                let mut attach_invoice = move |client_id, time_block_id| -> Option<()> {
                    let time_block = model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?;

                    let invoice = Invoice {
                        custom_id: Some("".to_owned()),
                        url: Some("".to_owned()),
                    };
                    // @TODO: Send request.
                    time_block.invoice = Some(invoice);
                    Some(())
                };
                attach_invoice(client_id, time_block_id);
            },
            Msg::DeleteInvoice(client_id, time_block_id) => {
                let mut delete_invoice = move |client_id, time_block_id| -> Option<()> {
                    let time_block = model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?;

                    if let Ok(true) = window().confirm_with_message(&format!("Invoice attached to Time Block \"{}\" will be deleted.", time_block.name)) {
                        time_block.invoice = None;
                        // @TODO: Send request.
                    }
                    Some(())
                };
                delete_invoice(client_id, time_block_id);
            },

            Msg::InvoiceCustomIdChanged(client_id, time_block_id, custom_id) => {
                let mut set_invoice_custom_id = move |client_id, time_block_id, custom_id| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .invoice.as_mut()?
                        .custom_id = Some(custom_id))
                };
                set_invoice_custom_id(client_id, time_block_id, custom_id);
            },
            Msg::SaveInvoiceCustomId(client_id, time_block_id) => {
                // @TODO: Send request.
            },

            Msg::InvoiceUrlChanged(client_id, time_block_id, url) => {
                let mut set_invoice_url = move |client_id, time_block_id, url| -> Option<()> {
                    Some(model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .time_blocks
                        .get_mut(&time_block_id)?
                        .invoice.as_mut()?
                        .url = Some(url))
                };
                set_invoice_url(client_id, time_block_id, url);
            },
            Msg::SaveInvoiceUrl(client_id, time_block_id) => {
                // @TODO: Send request.
            },
        }
    }
    ```

1. `name_input` in `view_time_block`

    ```rust
    fn view_time_block(client_id: ClientId, time_block_id: TimeBlockId, time_block: &TimeBlock) -> Node<Msg> {
        div![C!["box"],
            div![C!["level", ...],
                input![C!["input", ..],
                    el_ref(&time_block.name_input),
                    ...
    ```

## Time Tracker

![Time Tracker - Time Tracker update](/static/images/time_tracker_update_time_tracker.gif)

`src/page/time_tracker.rs`

1. `update`

    ```rust
    pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
        match msg {
            Msg::ClientsFetched(Ok(clients)) => {
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
                model.errors.clear();
            },

            Msg::Start(client_id, project_id) => {
                let mut start_time_entry = move |client_id, project_id| -> Option<()> {
                    let time_entries = &mut model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries;

                    let previous_name = time_entries
                        .iter()
                        .next_back()
                        .map(|(_, time_entry)| time_entry.name.to_owned());

                    let time_entry_id = TimeEntryId::new();
                    let time_entry = TimeEntry {
                        name: previous_name.unwrap_or_default(),
                        started: chrono::Local::now(),
                        stopped: None,
                        change: None,
                    };
                    // @TODO: Send request.
                    time_entries.insert(time_entry_id, time_entry);

                    Some(())
                };
                start_time_entry(client_id, project_id);
            },
            Msg::Stop(client_id, project_id) => {
                let mut stop_time_entry = move |client_id, project_id| -> Option<()> {
                    let (time_entry_id, time_entry) = model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .iter_mut()
                        .find(|(_, time_entry)| time_entry.stopped.is_none())?;
                    
                    time_entry.stopped = Some(chrono::Local::now());
                    // @TODO: Send request.
                    Some(())
                };
                stop_time_entry(client_id, project_id);
            },

            Msg::DeleteTimeEntry(client_id, project_id, time_entry_id) => {
                let mut delete_time_entry = move |client_id, project_id, time_entry_id| -> Option<()> {
                    let time_entries = &mut model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries;

                    let time_entry_name = &time_entries.get_mut(&time_entry_id)?.name;

                    if let Ok(true) = window().confirm_with_message(&format!("Time Entry \"{}\" will be deleted.", time_entry_name)) {
                        time_entries.remove(&time_entry_id);
                        // @TODO: Send request.
                    }
                    Some(())
                };
                delete_time_entry(client_id, project_id, time_entry_id);
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
                set_time_entry_name(name);
            },
            Msg::SaveTimeEntryName(client_id, project_id, time_entry_id) => {
                // @TODO: Send request.
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
                set_time_entry_change(TimeEntryChange::StoppedTime(time));
            },

            Msg::SaveTimeEntryChange(client_id, project_id, time_entry_id) => {
                let mut save_time_entry_change = move || -> Option<()> {
                    let time_entry = model
                        .clients
                        .loaded_mut()?
                        .get_mut(&client_id)?
                        .projects
                        .get_mut(&project_id)?
                        .time_entries
                        .get_mut(&time_entry_id)?;

                    match time_entry.change.take()? {
                        TimeEntryChange::StartedDate(date) => {
                            let date = chrono::NaiveDate::parse_from_str(&date, "%F").ok()?;
                            let time = time_entry.started.time();
                            time_entry.started = Local.from_local_date(&date).and_time(time).single()?;
                        }
                        TimeEntryChange::StartedTime(time) => {
                            let time = chrono::NaiveTime::parse_from_str(&time, "%X").ok()?;
                            let date = time_entry.started.naive_local().date();
                            time_entry.started = Local.from_local_date(&date).and_time(time).single()?;
                        }
                        TimeEntryChange::Duration(mut duration) => {
                            let negative = duration.chars().next()? == '-';
                            if negative {
                                duration.remove(0);
                            }
                            let mut duration_parts = duration.split(':');
                            let hours: i64 = duration_parts.next()?.parse().ok()?;
                            let minutes: i64 = duration_parts.next()?.parse().ok()?;
                            let seconds: i64 = duration_parts.next()?.parse().ok()?;

                            let mut total_seconds = hours * 3600 + minutes * 60 + seconds;
                            if negative {
                                total_seconds = -total_seconds;
                            }
                            let duration = chrono::Duration::seconds(total_seconds);
                            time_entry.stopped = Some(time_entry.started + duration);
                        }
                        TimeEntryChange::StoppedDate(date) => {
                            let date = chrono::NaiveDate::parse_from_str(&date, "%F").ok()?;
                            let time = time_entry.stopped?.time();
                            time_entry.stopped = Some(Local.from_local_date(&date).and_time(time).single()?);
                        }
                        TimeEntryChange::StoppedTime(time) => {
                            let time = chrono::NaiveTime::parse_from_str(&time, "%X").ok()?;
                            let date = time_entry.stopped?.naive_local().date();
                            time_entry.stopped = Some(Local.from_local_date(&date).and_time(time).single()?);
                        }
                    }
                    // @TODO: Send request.
                    Some(())
                };
                save_time_entry_change();
            },

            Msg::OnSecondTick => (),
        }
    }
    ```

---

We've written relatively many lines and some boilerplate, but we should be proud, the app is almost fully functional!

We'll write GraphQL mutations in the next chapter so users can save their changes on the server.

