use pulldown_cmark::{self, CodeBlockKind, Event, Tag};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};
use uuid::Uuid;

fn main() {
    for path in html_and_text_files() {
        fs::remove_file(path).unwrap();
    }

    for path in markdown_files() {
        let markdown = fs::read_to_string(&path).unwrap();
        let (html, text_parts) = markdown_to_html_and_text_parts(&markdown);

        let parent_folder = format!(
            "generated_guides/{}",
            path.iter().nth_back(1).unwrap().to_str().unwrap()
        );
        if !Path::new(&parent_folder).exists() {
            fs::create_dir(&parent_folder).unwrap();
        }

        let file_stem = path.file_stem().unwrap().to_str().unwrap();

        let html_path = format!("{}/{}.html", parent_folder, file_stem);
        fs::write(html_path, html).unwrap();

        let text_path = format!("{}/{}.txt", parent_folder, file_stem);
        fs::write(text_path, text_parts.join(" ")).unwrap();
    }
}

fn html_and_text_files() -> Vec<PathBuf> {
    fs::read_dir("generated_guides")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                Some(fs::read_dir(path).unwrap())
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|entry| {
            let path = entry.unwrap().path();

            match path.extension().unwrap_or_default().to_str().unwrap() {
                "html" | "txt" => Some(path),
                _ => None,
            }
        })
        .collect()
}

fn markdown_files() -> Vec<PathBuf> {
    fs::read_dir("guides")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                Some(fs::read_dir(path).unwrap())
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().unwrap_or_default() == "md" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn markdown_to_html_and_text_parts(markdown: &str) -> (String, Vec<String>) {
    let parser = pulldown_cmark::Parser::new(markdown);

    let mut html = String::new();
    let text_parts = Rc::new(RefCell::new(Vec::<String>::new()));

    let parser = extract_lowercase_text(parser, text_parts.clone());
    let parser = transform_code_blocks(parser);
    let parser = add_details_el_key(parser);

    pulldown_cmark::html::push_html(&mut html, parser);
    (html, text_parts.replace(Vec::new()))
}

fn extract_lowercase_text<'a, I>(
    parser: I,
    text_parts: Rc<RefCell<Vec<String>>>,
) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let push_to_text_parts = move |text: &str| {
        text_parts.borrow_mut().push(text.to_lowercase());
    };

    parser.map(move |event| match event {
        Event::Text(text) => {
            push_to_text_parts(&text);
            Event::Text(text)
        },
        Event::Code(code) => {
            push_to_text_parts(&code);
            Event::Code(code)
        },
        Event::FootnoteReference(reference) => {
            push_to_text_parts(&reference);
            Event::FootnoteReference(reference)
        },
        _ => event,
    })
}

/// Transforms markdown code blocks to custom elements `<code-block lang="xx" code ="xx"></code-block>`.
#[allow(clippy::while_let_on_iterator)]
fn transform_code_blocks<'a, I>(
    mut parser: I,
) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut events = Vec::new();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(code_lang))) => {
                let mut attributes = Vec::<(&str, String)>::new();

                if !code_lang.is_empty() {
                    attributes.push(("lang", code_lang.to_string()));
                }

                let mut code = String::new();

                while let Some(event) = parser.next() {
                    match event {
                        Event::Text(text) => {
                            code.push_str(&text.to_string());
                        }
                        Event::End(Tag::CodeBlock(_)) => {
                            attributes.push(("code", code));
                            break;
                        }
                        _ => panic!("the closing code-block tag or text has to follow the opening code-block tag"),
                    }
                }

                let attributes: String = attributes
                    .into_iter()
                    .map(|(key, value)| {
                        format!(" {}=\"{}\"", key, value.replace('"', "&quot;"))
                    })
                    .collect();

                events.push(Event::Html(
                    format!("<code-block{}></code-block>", attributes).into(),
                ));
            },
            _ => events.push(event),
        }
    }

    events.into_iter()
}

fn add_details_el_key<'a, I>(parser: I) -> impl Iterator<Item = Event<'a>>
where
    I: Iterator<Item = Event<'a>>,
{
    parser.map(|event| match event {
        Event::Html(html) if html.contains("<details") => {
            let html = html.replacen(
                "<details",
                &format!("<details data-el-key=\"{}\" ", Uuid::new_v4()),
                1,
            );
            Event::Html(html.into())
        },
        _ => event,
    })
}
