use crate::Theme;
use dioxus::prelude::*;

#[component]
pub fn NavSection(theme: Signal<Theme>) -> Element {
    rsx! {
        nav {
            div {
                class: "nav-links",
                a { href: "#about", "About" }
                span { "|" }
                a { href: "#resume", "Resume" }
                span { "|" }
                a { href: "#contact", "Contact" }
                span { "|" }
                a {
                    id: "switch",
                    onclick: move |_| theme.set(theme().toggle()),
                    "{theme().toggle_text()}"
                }
            }
        }
    }
}
