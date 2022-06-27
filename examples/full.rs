use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*};
use dioxus_bulma::prelude::{
    tag::{Tag, Tags, TagLink},
    *,
};
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
            Columns {
                Column {
                    Button {
                        color: Colors::Success,
                        is_fullwidth: true,
                        onclick: move |_| {
                            toast.write().popup(ToastInfo::simple("success button clicked."));
                        }
                        "Success"
                    }
                }
                Column {
                    Button {
                        color: Colors::Info,
                        is_fullwidth: true,
                        onclick: move |_| {
                            toast.write().popup(ToastInfo::simple("info button clicked."));
                        }
                        "Info"
                    }
                }
                Column {
                    Button {
                        color: Colors::Warning,
                        is_fullwidth: true,
                        onclick: move |_| {
                            toast.write().popup(ToastInfo::simple("warning button clicked."));
                        }
                        "Warning"
                    }
                }
                Column {
                    Button {
                        color: Colors::Danger,
                        is_fullwidth: true,
                        onclick: move |_| {
                            toast.write().popup(ToastInfo::simple("danger button clicked."));
                        }
                        "Danger"
                    }
                }
            }
            br {}
            Columns {
                Column {
                    Progress {
                        color: Colors::Primary
                    }
                }
                Column {
                    Progress {
                        max: 100,
                        value: 30,
                        color: Colors::Info
                    }
                }
                Column {
                    Progress {
                        max: 100,
                        value: 85,
                        color: Colors::Dark
                    }
                }
            }
            br {}
            Columns {
                Column {
                    size: 4,
                    Tags {
                        Tag {
                            color: Colors::Warning,
                            size: Sizes::Medium,
                            "Rust"
                        }
                        Tag {
                            color: Colors::Link,
                            size: Sizes::Medium,
                            "Go"
                        }
                        Tag {
                            color: Colors::Info,
                            size: Sizes::Medium,
                            "Python"
                        }
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            "Ruby"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            "C++"
                        }
                    }
                }
                Column {
                    size: 3,
                    Tags {
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            deletable: true,
                            "React"
                        }
                        Tag {
                            color: Colors::Success,
                            size: Sizes::Medium,
                            deletable: true,
                            "Vue"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            deletable: true,
                            "Dioxus"
                        }
                    }
                }
                Column {
                    size: 4,
                    div {
                        class: "field is-grouped is-grouped-multiline",
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "crates.io"
                                }
                                Tag {
                                    color: Colors::Warning,
                                    size: Sizes::Medium,
                                    "v0.2.4"
                                }
                            }
                        }
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "docs"
                                }
                                Tag {
                                    color: Colors::Info,
                                    size: Sizes::Medium,
                                    "latest"
                                }
                            }
                        }
                    }
                }
                Column {
                    size: 1,
                    TagLink {
                        color: Colors::Link,
                        size: Sizes::Medium,
                        onclick: |_| {
                            toast.write().popup(ToastInfo::simple("clickable tag clicked."));
                        }
                        "Link"
                    }
                }
            }
        }
    })
}
