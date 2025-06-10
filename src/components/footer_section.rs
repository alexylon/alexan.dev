use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    rsx! {
        br {}
        a { href: "#top", "Go to top" }
        hr { class: "dashed" }

        footer {
            img { src: "https://forthebadge.com/images/badges/made-with-rust.svg" }
            br {}
            br {}
            p { "> Copyright 2025 Alexander Alexandrov" }
            p {
                a {
                    href: "https://github.com/alexylon/alexandroff.dev",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "source code"
                }
            }
        }
    }
}
