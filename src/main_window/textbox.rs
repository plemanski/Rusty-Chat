use dioxus::prelude::*;

pub struct TextBoxProps {
    default_text: String
}

pub fn textbox(cx: State<TextBoxProps>) -> Element {
    cx.render(rsx! {
        div{class: "textbox-container"},
    })
}