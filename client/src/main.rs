use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Header {}
        Frame {}
        Footer {}
    }
}

#[component]
pub fn Frame() -> Element {
    rsx! {
        div {
            class: "frame",
            "AAA"
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "header",
            div {
                class: "header-content",
                a {
                    class: "logo",
                    href: "/",
                    "research.okmtyuta.phd"
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",
            span {
                "© 2024"
            },
            a {
                class: "logo",
                href: "/",
                "research.okmtyuta.phd"
            }
        }
    }
}
