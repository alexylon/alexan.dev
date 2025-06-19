use dioxus::prelude::*;

#[component]
pub fn FooterSection() -> Element {
    rsx! {
        hr { class: "dashed" }

        footer {
            p {
                "Hosted on my Raspberry Pi and served with "
                a {
                    href: "https://github.com/alexylon/serve",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "serve"
                }
            }
            img { src: "https://forthebadge.com/images/badges/made-with-rust.svg" }
            br {}
            p { "© 2025 Alexander Alexandrov" }
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
