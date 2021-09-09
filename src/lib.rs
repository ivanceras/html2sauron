#![deny(warnings)]
#![deny(clippy::all)]
use sauron::prelude::*;
use sauron_syntax::html_to_syntax;
use sauron_syntax::Options;

#[macro_use]
extern crate log;

pub enum Msg {
    ChangeInput(String),
    Convert,
    ToggleMacro,
    ToggleArray,
}

pub struct App {
    input: String,
    output: String,
    options: Options,
    node_macro_checkbox: bool,
    array_checkbox: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            options: Options::default(),
            node_macro_checkbox: false,
            array_checkbox: false,
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        self.node_macro_checkbox = self.options.use_macro;
        self.array_checkbox = self.options.use_array;
        Cmd::none()
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeInput(input) => {
                self.input = input;
            }
            Msg::Convert => {
                self.output = html_to_syntax(&self.input, self.options).expect("must not error");
            }
            Msg::ToggleMacro => {
                self.options.use_macro = !self.options.use_macro;
            }
            Msg::ToggleArray => {
                self.options.use_array = !self.options.use_array;
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![styles([("display", "flex"), ("flex-direction", "column")])],
            vec![
                label(
                    vec![r#for("html_input")],
                    vec![text("Paste your html here:")],
                ),
                textarea(
                    vec![
                        id("html_input"),
                        styles([("height", px(400))]),
                        on_input(|input| Msg::ChangeInput(input.value)),
                        value(&self.input),
                    ],
                    vec![],
                ),
                div(
                    vec![styles([
                        ("display", "flex"),
                        ("flex-direction", "row"),
                        ("margin-bottom", "50px"),
                    ])],
                    vec![
                        button(
                            vec![styles([("width", px(200))]), on_click(|_| Msg::Convert)],
                            vec![text("Convert >> ")],
                        ),
                        div(
                            vec![styles([
                                ("display", "flex"),
                                ("flex-direction", "row"),
                                ("align-items", "center"),
                                ("padding", "10px"),
                            ])],
                            vec![
                                input(
                                    vec![
                                        r#type("checkbox"),
                                        id("use_macro_check"),
                                        on_click(|_| Msg::ToggleMacro),
                                        checked(self.options.use_macro),
                                    ],
                                    vec![],
                                ),
                                label(
                                    vec![r#for("use_macro_check")],
                                    vec![text("Use node! macro")],
                                ),
                                input(
                                    vec![
                                        r#type("checkbox"),
                                        id("use_array_check"),
                                        on_click(|_| Msg::ToggleArray),
                                        disabled(self.options.use_macro),
                                        checked(self.options.use_array),
                                    ],
                                    vec![],
                                ),
                                label(vec![r#for("use_array_check")], vec![text("Use array")]),
                            ],
                        ),
                    ],
                ),
                label(
                    vec![r#for("sauron_syntax")],
                    vec![
                        a(
                            vec![href("https://github.com/ivanceras/sauron")],
                            vec![text("Sauron")],
                        ),
                        text(" view code:"),
                    ],
                ),
                textarea(
                    vec![
                        id("sauron_syntax"),
                        styles([("height", px(400))]),
                        value(&self.output),
                    ],
                    vec![],
                ),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).expect("must be initiated");
    info!("started");
    Program::mount_to_body(App::default());
}
