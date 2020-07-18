//#![deny(warnings)]
use sauron::prelude::*;
use sauron_syntax::html_to_syntax;
use wasm_bindgen::prelude::*;

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
}

impl App {
    pub fn new() -> Self {
        App {
            input: String::new(),
            output: String::new(),
            use_macro: false,
        }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        div!(
            [styles([("display", "flex"), ("flex-direction", "column")])],
            [
                label!([for_("html_input")], [text("Paste your html here:")]),
                textarea!(
                    [
                        id("html_input"),
                        styles([("height", px(400)),]),
                        on_input(|input| Msg::ChangeInput(input.value)),
                        value(&self.input),
                    ],
                    []
                ),
                div!(
                    [styles([
                        ("display", "flex"),
                        ("flex-direction", "row"),
                        ("margin-bottom", "50px"),
                    ])],
                    [
                        div!(
                            [styles([
                                ("display", "flex"),
                                ("flex-direction", "row"),
                                ("align-items", "center"),
                                ("padding", "10px"),
                            ])],
                            [
                                input!(
                                    [
                                        type_("checkbox"),
                                        id("use_macro_check"),
                                        on_click(|_| Msg::ToggleMacro)
                                    ],
                                    []
                                )
                                .add_attributes(attrs_flag([(
                                    "checked",
                                    "checked",
                                    self.use_macro
                                )])),
                                label!([for_("use_macro_check")], [text("Use macro")])
                            ]
                        ),
                        button!(
                            [styles([("width", px(200))]), on_click(|_| Msg::Convert)],
                            [text("Convert >> ")]
                        ),
                    ]
                ),
                label!(
                    [for_("sauron_syntax")],
                    [
                        a!(
                            [href("https://github.com/ivanceras/sauron")],
                            [text("Sauron")]
                        ),
                        text(" view code:")
                    ]
                ),
                textarea!(
                    [
                        id("sauron_syntax"),
                        styles([("height", px(400))]),
                        value(&self.output)
                    ],
                    []
                ),
            ]
        )
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
                self.output = html_to_syntax(&self.input, self.use_macro).expect("must not error");
            }
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).expect("must be initiated");
    info!("started");
    Program::mount_to_body(App::new());
}
