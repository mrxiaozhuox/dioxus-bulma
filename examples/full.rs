use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*};
use dioxus_bulma::prelude::*;
use dioxus_toast::{ToastFrame, ToastInfo, ToastManager};

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    dioxus::desktop::launch_cfg(App, |config| {
        config.with_window(|win| {
            win.with_title("Dioxus Bulma - Full Example")
                .with_inner_size(LogicalSize::new(1200, 700))
        })
    })
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    let cdn_url = dioxus_bulma::get_bulma_cdn();
    cx.render(rsx! {
        link { rel: "stylesheet", href: "{cdn_url}" }
        ToastFrame {
            manager: toast
        }
        br {}
        Container {
            fluid: true,
            div {
                Breadcurmb {
                    ul {
                        li { a { href: "#", "DioxusLabs" } }
                        li { a { href: "#", "dioxus" } }
                        li { a { href: "#", "dioxus-bulma" } }
                    }
                }
            }
            br {}
            div {
                Button {
                    color: Colors::Info,
                    onclick: move |_| {
                        toast.write().popup(ToastInfo::simple("Colors::Info button clicked."));
                    }
                    "Hello World"
                }
            }
        }
    })
}
