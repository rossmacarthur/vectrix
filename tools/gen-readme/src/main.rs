use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{bail, Context, Result};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, LinkType, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark_with_options;
use serde::Serialize;
use tinytemplate::TinyTemplate;

const ROOT: &str = env!("CARGO_WORKSPACE_DIR");
const VERSION: &str = "0.2";

/// Render Markdown events as Markdown.
fn to_cmark<'a, I, E>(events: I) -> Result<String>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
{
    let mut buf = String::new();
    cmark_with_options(
        events,
        &mut buf,
        None,
        pulldown_cmark_to_cmark::Options {
            code_block_token_count: 3,
            ..Default::default()
        },
    )?;
    Ok(buf)
}

const TYPES: &[(&str, &str)] = &[
    ("Matrix", "struct"),
    ("Vector", "struct"),
    ("RowVector", "struct"),
    ("matrix", "macro"),
    ("vector", "macro"),
    ("row_vector", "macro"),
];

fn type_from_path(path: &[&str]) -> Option<&'static str> {
    TYPES
        .iter()
        .find_map(|(n, ty)| (*n == path[0]).then(|| *ty))
}

fn url_from_path(path: &[&str]) -> Option<String> {
    match *path {
        ["core", "fmt", trt] => Some(format!(
            "https://doc.rust-lang.org/std/fmt/trait.{}.html",
            trt,
        )),
        ["core", "iter", "FromIterator", "from_iter"] => Some(String::from(
            "https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter",
        )),
        ["Zero", "zero"] => Some(format!(
            "https://docs.rs/vectrix/{}/vectrix/traits/trait.Zero.html#tymethod.zero",
            VERSION
        )),
        [name] => {
            let ty = type_from_path(path)?;
            Some(format!(
                "https://docs.rs/vectrix/{}/vectrix/{}.{}.html",
                VERSION, ty, name
            ))
        }
        [name, segment] => {
            let ty = type_from_path(path)?;
            Some(format!(
                "https://docs.rs/vectrix/{}/vectrix/{}.{}.html#method.{}",
                VERSION, ty, name, segment
            ))
        }
        _ => None,
    }
}

fn gen_contents() -> Result<String> {
    let path = PathBuf::from_iter([ROOT, "src", "lib.rs"]);
    let lib = fs::read_to_string(path)?;

    let i = lib.find("#![no_std]").context("find no_std attribute")?;

    let text = lib[..i].replace("\n//! ", "\n").replace("//!", "");
    let mut parser = Parser::new_ext(&text, Options::all()).peekable();
    let mut events = Vec::new();
    let mut urls = BTreeMap::new();
    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::Borrowed("")))) => {
                let new_tag = Tag::CodeBlock(CodeBlockKind::Fenced("rust".into()));
                let start = Event::Start(new_tag.clone());
                events.push(start);
                let mut new_text = String::new();
                while let Event::Text(text) = parser.next().unwrap() {
                    for line in text.lines().filter(|line| !line.starts_with('#')) {
                        new_text.push_str(line);
                        new_text.push('\n');
                    }
                }
                events.push(Event::Text(new_text.into()));
                events.push(Event::End(new_tag));
            }
            Event::Text(CowStr::Borrowed("[")) => {
                let code = match parser.next().unwrap() {
                    Event::Code(CowStr::Borrowed(code)) => code,
                    event => panic!("unexpected event `{:?}`", event),
                };
                assert!(matches!(
                    parser.next().unwrap(),
                    Event::Text(CowStr::Borrowed("]"))
                ));

                let path: Vec<_> =
                    if matches!(parser.peek(), Some(Event::Text(CowStr::Borrowed("[")))) {
                        assert!(matches!(
                            parser.next().unwrap(),
                            Event::Text(CowStr::Borrowed("["))
                        ));
                        let code = match parser.next().unwrap() {
                            Event::Code(CowStr::Borrowed(code)) => code,
                            event => panic!("unexpected event `{:?}`", event),
                        };
                        assert!(matches!(
                            parser.next().unwrap(),
                            Event::Text(CowStr::Borrowed("]"))
                        ));
                        code.split("::")
                            .map(|s| s.trim_end_matches(|c: char| !c.is_alphanumeric()))
                            .collect()
                    } else {
                        let code = match code.find('<') {
                            Some(i) => &code[..i],
                            None => code,
                        };
                        code.split("::")
                            .map(|s| s.trim_end_matches(|c: char| !c.is_alphanumeric()))
                            .collect()
                    };

                match url_from_path(&path) {
                    Some(url) => {
                        let ty = type_from_path(&path)
                            .map(|ty| format!("{}.", ty))
                            .unwrap_or_else(|| String::from(""));
                        urls.entry(url.clone())
                            .or_insert_with(|| format!("{}{}", ty, path.join("::")));
                        let link = Tag::Link(LinkType::Reference, url.into(), "".into());
                        events.push(Event::Start(link.clone()));
                        events.push(Event::Code(CowStr::Borrowed(code)));
                        events.push(Event::End(link));
                    }
                    None => println!("unknown path {:?}", path),
                }
            }
            event => events.push(event),
        }
    }

    let mut cmark = to_cmark(events.into_iter())?;
    cmark.push_str("\n\n");
    for (url, name) in urls {
        cmark = cmark.replace(&format!("({})", url), &format!("[{}]", name));
        cmark.push_str(&format!("[{}]: {}\n", name, url));
    }
    Ok(cmark)
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let borrowed: Vec<_> = args.iter().map(String::as_str).collect();
    let check = match borrowed.as_slice() {
        &["--check"] => true,
        &[] => false,
        what => bail!("unrecognized command line argument(s): {:?}", what),
    };

    #[derive(Serialize)]
    struct Context {
        contents: String,
    }

    const TEMPLATE_NAME: &str = "readme";
    const TEMPLATE: &str = include_str!("../TEMPLATE.md");

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template(TEMPLATE_NAME, TEMPLATE)?;
    let contents = gen_contents()?;
    let path = PathBuf::from_iter([ROOT, "README.md"]);
    let rendered = tt.render(TEMPLATE_NAME, &Context { contents })?;

    let current = fs::read_to_string(&path)?;
    if current == rendered {
        println!("README is up to date!");
    } else if check {
        bail!("README is not up to date!");
    } else {
        fs::write(path, rendered)?;
        println!("README was updated!");
    }

    Ok(())
}
