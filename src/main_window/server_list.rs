use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct ServerListProps {
    server_name: String
}

//Need to fetch channel list and server name from config

pub fn server_list(cx: Scope<ServerListProps>) -> Element {
    cx.render(rsx! {
        div{ class: "server-list container",
            background_color: "#363057",
            width: "225px",
            height: "100%",
            padding: "15px",
                div{ class: "title container", 
                    h2{ "{cx.props.server_name}" }
                },
                div{ class: "channel container",
                    ul{ class: "channel-list",
                    padding_bottom: "5px",
                    list_style: "none",
                    li{"Channel1"},
                    li{"Channel2"},
                    li{"Channel3"}
                }
            }
        }
    })
}