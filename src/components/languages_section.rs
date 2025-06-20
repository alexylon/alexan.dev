use crate::components::data::LANGUAGES;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            class: "languages",
            h3 { "## Spoken Languages" }
            ul {
                class: "languages-list",
                {
                    LANGUAGES.iter().map(|lang| rsx! {
                        li {
                            span { "{lang.name}" }
                            span { "{lang.level}" }
                        }
                    })
                }
            }
        }
    }
}
