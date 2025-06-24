use crate::components::data::LANGUAGES;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    rsx! {
        section {
            class: "languages-section",
            h2 { "Spoken Languages" }
            ul {
                class: "languages-list list-disc",
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
