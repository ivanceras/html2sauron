//#![deny(warnings)]
use sauron::parser::convert_html_to_syntax;
use sauron::{
    html::units::*,
    html::{attributes::*, events::*, *},
    input, Cmd, Component, Node, Program, *,
};
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
                        oninput(|input| Msg::ChangeInput(input.value)),
                        value(&self.input),
                    ],
                    []
                ),
                div!(
                    [styles([("display", "flex"), ("flex-direction", "row")])],
                    [
                        button!(
                            [styles([("width", px(200))]), onclick(|_| Msg::Convert)],
                            [text("Convert >> ")]
                        ),
                        input!(
                            [
                                type_("checkbox"),
                                id("use_macro_check"),
                                onclick(|_| Msg::ToggleMacro)
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
                label!([for_("sauron_syntax")], [text("Sauron view code")]),
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
                /*
                self.output = convert_html_to_syntax(&input, self.use_macro);
                log::debug!("self.output: {}", self.output);
                */
            }
            Msg::Convert => {
                self.output = convert_html_to_syntax(&self.input, self.use_macro);
            }
            Msg::ToggleMacro => {
                self.use_macro = !self.use_macro;
                self.output = convert_html_to_syntax(&self.input, self.use_macro);
            }
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).expect("must be initiated");
    Program::mount_to_body(App::new());
}
