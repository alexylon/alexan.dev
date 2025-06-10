use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            id: "languages",
            h3 { "## Spoken Languages" }
            ul {
                class: "skill-list",
                li {
                    span { "Bulgarian" }
                    span { "+++++" }
                }
                li {
                    span { "English" }
                    span { "++++-" }
                }
                li {
                    span { "Italian" }
                    span { "+++--" }
                }
                li {
                    span { "Russian" }
                    span { "++---" }
                }
                li {
                    span { "Greek" }
                    span { "+----" }
                }
            }
        }
    }
}
