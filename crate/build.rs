use pulldown_cmark::{self, Event, Tag};
use std::fs;
use std::path::PathBuf;

fn main() {
    for path in html_files() {
        fs::remove_file(path).unwrap();
    }

    for path in markdown_files() {
        let markdown = fs::read_to_string(&path).unwrap();
        let html = markdown_to_html(&markdown);

        let html_path = format!("generated_guides/{}.html", path.file_stem().unwrap().to_str().unwrap());
        fs::write(html_path, html).unwrap();
    }
}

fn html_files() -> Vec<PathBuf> {
    fs::read_dir("generated_guides")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().unwrap_or_default() == "html" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn markdown_files() -> Vec<PathBuf> {
    fs::read_dir("guides")
        .unwrap()
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

fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let parser_with_syntax_highlighting = highlight_syntax(parser);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser_with_syntax_highlighting);
    html
}

fn highlight_syntax<'a, I>(parser: I) -> impl Iterator<Item = Event<'a>>
    where
        I: Iterator<Item = Event<'a>>,
{
    parser
//    parser.map(|event| {
//        match event {
////            Event::Start(Tag::CodeBlock(code_block)) => {
////                Event::Start(Tag::CodeBlock("ZXC".into()))
////            }
//            Event::Text(text) => {
//                Event::Text("VBN".into())
//            }
//            _ => event
//        }
//    })
}



