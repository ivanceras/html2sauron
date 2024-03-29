use crate::Options;
use sauron::prelude::*;
use sauron_markdown::html_parser;
use std::{fmt, fmt::Write};

/// A trait to convert html string into sauron view syntax
pub trait ToSyntax {
    /// convert the html string into sauron view syntax
    fn to_syntax(&self, buffer: &mut dyn Write, options: Options, indent: usize) -> fmt::Result;
}

impl<MSG: 'static> ToSyntax for Node<MSG> {
    fn to_syntax(&self, buffer: &mut dyn Write, options: Options, indent: usize) -> fmt::Result {
        match self {
            Node::Text(text) => {
                if options.use_macro {
                    write!(buffer, "\"{}\"", text.text)
                } else {
                    write!(buffer, "text(\"{}\")", text.text)
                }
            }
            Node::Element(element) => {
                write!(buffer, "{}", make_indent(indent))?;
                element.to_syntax(buffer, options, indent)
            }
            Node::Comment(comment) => {
                write!(buffer, "comment(\"{}\")", comment)
            }
        }
    }
}

impl<MSG: 'static> ToSyntax for Attribute<MSG> {
    fn to_syntax(&self, buffer: &mut dyn Write, options: Options, indent: usize) -> fmt::Result {
        for att_value in self.value() {
            if options.use_macro {
                match att_value {
                    AttributeValue::Simple(simple) => {
                        if let Some(_ns) = self.namespace() {
                            write!(buffer, "xlink::{}", self.name().to_string(),)?;
                            write!(buffer, "=")?;
                            simple.to_syntax(buffer, options, indent)?;
                        } else if let Some(att_name) = html_parser::attribute_function(self.name())
                        {
                            write!(buffer, "{}", att_name)?;
                            write!(buffer, "=")?;
                            simple.to_syntax(buffer, options, indent)?;
                        } else {
                            write!(buffer, "{}=", self.name().to_string(),)?;
                            simple.to_syntax(buffer, options, indent)?;
                        }
                    }
                    AttributeValue::Style(styles_att) => {
                        write!(buffer, "style=\"")?;
                        for s_att in styles_att {
                            write!(buffer, "{};", s_att)?;
                        }
                        write!(buffer, "\"")?;
                    }
                    _ => (),
                }
            } else {
                match att_value {
                    AttributeValue::Simple(simple) => {
                        if let Some(_ns) = self.namespace() {
                            write!(buffer, "xlink_{}", self.name().to_string(),)?;
                            write!(buffer, "(")?;
                            simple.to_syntax(buffer, options, indent)?;
                            write!(buffer, ")")?;
                        } else if let Some(att_name) = html_parser::attribute_function(self.name())
                        {
                            write!(buffer, "{}", att_name)?;
                            write!(buffer, "(")?;
                            simple.to_syntax(buffer, options, indent)?;
                            write!(buffer, ")")?;
                        } else {
                            write!(buffer, r#"attr("{}","#, self.name().to_string(),)?;
                            simple.to_syntax(buffer, options, indent)?;
                            write!(buffer, ")")?;
                        }
                    }
                    AttributeValue::Style(styles_att) => {
                        write!(buffer, "style(\"")?;
                        for s_att in styles_att {
                            write!(buffer, "{};", s_att)?;
                        }
                        write!(buffer, "\")")?;
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}

impl ToSyntax for Value {
    fn to_syntax(&self, buffer: &mut dyn Write, _options: Options, _indent: usize) -> fmt::Result {
        if let Some(v_str) = self.as_str() {
            write!(buffer, "\"{}\"", v_str)?;
        }
        Ok(())
    }
}

impl<MSG: 'static> ToSyntax for Element<MSG> {
    fn to_syntax(&self, buffer: &mut dyn Write, options: Options, indent: usize) -> fmt::Result {
        let self_closing = html_parser::is_self_closing(self.tag());
        if options.use_macro {
            write!(buffer, "<{}", self.tag())?;
            for attr in self.get_attributes().iter() {
                write!(buffer, " ")?;
                attr.to_syntax(buffer, options, indent)?;
            }
            if !self_closing {
                write!(buffer, ">")?;
            }
            let children = self.get_children();
            let first_child = children.get(0);
            let is_first_child_text_node = first_child.map(|node| node.is_text()).unwrap_or(false);

            let is_lone_child_text_node = children.len() == 1 && is_first_child_text_node;

            if is_lone_child_text_node {
                first_child.unwrap().to_syntax(buffer, options, indent)?;
            } else {
                // otherwise print all child nodes with each line and indented
                for child in self.get_children() {
                    writeln!(buffer)?;
                    child.to_syntax(buffer, options, indent + 1)?;
                }
            }
            if self_closing {
                write!(buffer, "/>")?;
            } else {
                if is_lone_child_text_node || children.is_empty() {
                    // no new line if a lone child text node or empty
                } else {
                    write!(buffer, "\n{}", make_indent(indent))?;
                }
                write!(buffer, "</{}>", self.tag())?;
            }
        } else {
            write!(buffer, "{}(", self.tag())?;
            if options.use_array {
                write!(buffer, "[")?;
            } else {
                write!(buffer, "vec![")?;
            }
            let total_attrs = self.get_attributes().len();
            for (i, attr) in self.get_attributes().iter().enumerate() {
                attr.to_syntax(buffer, options, indent)?;
                if i < total_attrs - 1 {
                    write!(buffer, ", ")?;
                }
            }
            write!(buffer, "],")?;
            if options.use_array {
                write!(buffer, "[")?;
            } else {
                write!(buffer, "vec![")?;
            }
            let children = self.get_children();
            let first_child = children.get(0);
            let is_first_child_text_node = first_child.map(|node| node.is_text()).unwrap_or(false);

            let is_lone_child_text_node = children.len() == 1 && is_first_child_text_node;

            if is_lone_child_text_node {
                first_child.unwrap().to_syntax(buffer, options, indent)?;
            } else {
                // otherwise print all child nodes with each line and indented
                for child in self.get_children() {
                    writeln!(buffer)?;
                    child.to_syntax(buffer, options, indent + 1)?;
                    write!(buffer, ",")?;
                }
            }
            if is_lone_child_text_node || children.is_empty() {
                // no new line if a lone child text node or empty
            } else {
                write!(buffer, "\n{}", make_indent(indent))?;
            }
            write!(buffer, "])")?;
        }
        Ok(())
    }
}

/// convenient function to create indent
fn make_indent(n: usize) -> String {
    "    ".repeat(n)
}
