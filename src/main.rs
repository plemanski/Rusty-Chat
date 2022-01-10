use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(app, |c|{
        c.with_window(|w| {
            w.with_resizable(true).with_inner_size(
                dioxus::desktop::wry::application::dpi::LogicalSize::new(600.00, 800.00),
            )
        })
    });
}

fn app (cx: Scope) -> Element {
    cx.render(rsx! (
        div{ class: "main-window", 
            div{ class: "container mx-auto",
                
            }
    }
    ))
}