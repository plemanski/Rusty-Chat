use dioxus::prelude::*;
use irc::client::prelude::Message;

#[derive(Props, PartialEq)]
pub struct TextBoxProps{
    messages: Vec<Message>
}

pub fn textbox(cx: Scope<TextBoxProps>) -> Element {
    cx.render(rsx!{
        div{ class: "textbox-container"},
        ul{class: "message-list",
        cx.props.messages.iter().map(|message| rsx!(
            li{class:"messages",
            "{message}"
            }
        ))
        }
    }
)}