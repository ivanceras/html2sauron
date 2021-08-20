#![deny(warnings)]
use sauron::prelude::*;
use sauron_syntax::html_to_syntax;

#[macro_use]
extern crate log;

pub enum Msg {
    ChangeInput(String),
    Convert,
    ToggleMacro,
}

pub struct App {
    input: String,
    output: String,
    use_macro: bool,
    node_checkbox: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            output: String::new(),
            use_macro: true,
            node_checkbox: true,
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self, _: Program<Self, Msg>) -> Cmd<Self, Msg> {
        self.node_checkbox = self.use_macro;
        Cmd::none()
    }
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ChangeInput(input) => {
                self.input = input.clone();
            }
            Msg::Convert => {
                self.output = html_to_syntax(&self.input, self.use_macro).expect("must not error");
            }
            Msg::ToggleMacro => {
                self.use_macro = !self.use_macro;
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
                                        checked(self.use_macro),
                                    ],
                                    vec![],
                                ),
                                label(
                                    vec![r#for("use_macro_check")],
                                    vec![text("Use node! macro")],
                                ),
                            ],
                        ),
                        button(
                            vec![styles([("width", px(200))]), on_click(|_| Msg::Convert)],
                            vec![text("Convert >> ")],
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
    Program::mount_to_body(App::new());
}
