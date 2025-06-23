use crate::components::data::{PROFILE};
use crate::Theme;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection(theme: Signal<Theme>) -> Element {
    rsx! {
        section {
            class: "header",
            div {
                class: "container",
                div {
                    h1 { "{PROFILE.name}" }
                    h3 { "{PROFILE.title}" }
                }
                a {
                    class: "theme-switch",
                    onclick: move |_| theme.set(theme().toggle()),
                    img {
                        src: "{theme().icon_theme()}",
                        alt: "Theme Icon",
                        width: "24",
                    }
                }
            }
        }
    }
}
