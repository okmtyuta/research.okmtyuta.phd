use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    About {},
}

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            Router::<Route> {}
        }
    });
}

#[component]
fn About() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Header {}
        Frame {
            h1 { "Overview" }
            p {
                " 広い意味で数理科学と情報科学を勉強しています。とくに 数理物理学（量子力学），確率統計学，代数学，幾何学，プログラミング言語，機械学習 ，暗号理論に興味があります。いずれの分野についてもその理論構造に強い関心があります。また，UI/UXデザインやグラフィックデザインにも興味があります。"
            }
            p {
                "I am majored in mathematical sciences and information technologies in a broad sense. I am particularly interested in quantum mechanics , probability and statistics, algebra, geometry, programming languages, machine learning , and cryptology . For each areas, I am strongly interested in their theoretical structure."
            }
            h1 { "Educations" }
            p {
                "I have studied Mathematical Physics in undergraduate school. In graduate school, I am going to study topological data analysis and statistical machine learning."
            }
            ul {
                li {
                    "Bachelor's degree. Department of Mathematics, Faculty of Science, University of Tokyo : 2020 April - 2024 March."
                }
                li {
                    "Master's degree. Graduate School of Informatics, Kyoto University: 2024 April - 2026 March."
                }
            }
            h1 { "Experiences" }
            p {
                "学外ではソフトウェアエンジニアとして活動してきました。機械学習やUI/UXデザインの領域でも価値を出していきたいと思っています。"
            }
            p {
                "I have worked as software engineer. From now on, I would like to be active in the areas of not only software, but also Machine Learning and UI/UX design."
            }
            ul {
                li { "Rakuten Group, Inc. : August 2022, Application Engineer." }
                li { "Net Protections, Inc. : October 2022 - April 2023, Application Engineer." }
                li { "PKSHA Technology, Inc. : March 2023 - April 2023, Software Engineer." }
                li { "PKSHA Technology, Inc. : October 2023 - Current, Software Engineer." }
            }
        }
        Footer {}
    }
}

#[derive(PartialEq, Clone, Props)]
struct FrameProps {
    children: Element,
}

#[component]
pub fn Frame(props: FrameProps) -> Element {
    rsx! {
        div { class: "frame", {props.children} }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "header-content",
                a { class: "logo", href: "/", "research.okmtyuta.phd" }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            span { "© 2024" }
            a { class: "logo", href: "/", "research.okmtyuta.phd" }
        }
    }
}
