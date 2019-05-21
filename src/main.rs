//#![deny(warnings)]
extern crate html5ever;

use html5ever::driver;
use html5ever::local_name;
use html5ever::namespace_url;
use html5ever::ns;
use html5ever::parse_fragment;
use html5ever::rcdom::Node;
use html5ever::rcdom::{NodeData, RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::QualName;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod attributes;
mod tags;

/// Initializes an HTML fragment parser.
///
/// Ammonia conforms to the HTML5 fragment parsing rules,
/// by parsing the given fragment as if it were included in a <div> tag.
fn make_parser() -> driver::Parser<RcDom> {
    parse_fragment(
        RcDom::default(),
        driver::ParseOpts::default(),
        QualName::new(None, ns!(html), local_name!("div")),
        vec![],
    )
}

fn padd(n: i32) -> String {
    let mut buffer = String::new();
    for _ in 0..n {
        buffer += "    ";
    }
    buffer
}

fn process_node(node: &Node, indent: i32, opt: &Opt) -> String {
    let mut buffer = String::new();
    // process the children first
    let mut child_buffer = vec![];
    for child in node.children.borrow().iter() {
        let child_elm = process_node(child, indent + 1, opt);
        if !child_elm.trim().is_empty() {
            child_buffer.push(child_elm);
        }
    }
    match &node.data {
        NodeData::Element { name, attrs, .. } => {
            let tag = name.local.to_string();
            let corrected_tag = if tags::is_valid_tag(&tag) {
                tag
            } else {
                // replace custom tag with div
                "div".to_string()
            };
            let mut elm_buffer = String::new();
            elm_buffer += &format!("{}([", corrected_tag);
            let mut att_buffer = vec![];
            for att in attrs.borrow().iter() {
                let key = att.name.local.to_string();
                let value = att.value.to_string();
                if opt.trim_invalid && !attributes::is_valid(&key) {
                    // exclude
                } else {
                    att_buffer.push(attributes::format(&key, &value, opt));
                }
            }
            elm_buffer += &att_buffer.join(", ");
            elm_buffer += &format!("],[\n");
            elm_buffer += &format!(
                "{}{}",
                padd(indent),
                child_buffer.join(&format!(",\n{}", padd(indent)))
            );
            elm_buffer += &format!("\n{}])", padd(indent - 1));
            buffer += &elm_buffer
        }

        NodeData::Text { contents } => {
            let text = contents.borrow().trim().to_string();
            if !text.is_empty() {
                buffer += &format!(r#"text("{}")"#, text);
            }
        }
        _ => {
            buffer += &child_buffer.join(",");
        }
    }
    buffer
}

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "html2sauron")]
pub struct Opt {
    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Trim invalid attributes
    #[structopt(
        short = "trim",
        long = "trim_invalid",
        parse(try_from_str),
        default_value = "true"
    )]
    trim_invalid: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    /// Remove classes that is prefix with argument
    #[structopt(short = "r", long = "remove-classes-with-prefix")]
    remove_class_with_prefix: Option<String>,
}

fn read_file(file: &PathBuf) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn write_to_file(file: &PathBuf, sauron: &str) -> io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(sauron.as_bytes())?;
    Ok(())
}

pub fn html2sauron(html: &str, opt: &Opt) -> String {
    let parser = make_parser();
    let fragment = parser.one(html);
    process_node(&fragment.document, 0, opt)
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let html = read_file(&opt.file)?;
    let sauron = html2sauron(&html, &opt);
    if let Some(output) = &opt.output {
        write_to_file(output, &sauron)?;
    } else {
        println!("{}", sauron);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let html = "<a>Hello world</a><div class='btn large'>link to here</div><utag>how to deal with this</utag>";
        let view = html2sauron(html, &Opt::default());
        println!("{}", view);
        assert_eq!(
            view,
            r#"
html([],[
    a([],[
        text("Hello world")
    ]),
    div([class("btn large")],[
        text("link to here")
    ]),
    div([],[
        text("how to deal with this")
    ])
])
                   "#
            .trim()
            .to_string()
        );
    }

    #[test]
    #[should_panic]
    fn test_with_invalid_tag() {
        let html = "<a>Hello world</a><div class='btn large'>link to here</div><utag>how to deal with this</utag>";
        let view = html2sauron(html, &Opt::default());
        println!("{}", view);
        assert_eq!(
            view,
            r#"
html([],[
    a([],[
        text("Hello world")
    ]),
    div([class("btn large")],[
        text("link to here")
    ]),
    utag([],[
        text("how to deal with this")
    ])
])
                   "#
            .trim()
            .to_string()
        );
    }
}
