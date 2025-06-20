use crate::components::data::CAREER_ENTRIES;
use dioxus::prelude::*;

#[component]
pub fn CareerSection() -> Element {
    rsx! {
        section {
            class: "career",
            h3 { "## Career History" }
            {CAREER_ENTRIES.iter().map(|entry| rsx! {
                h4 {
                    b { "{entry.title}" },
                    i { " • {entry.company_and_period}" }
                }
                ul {
                    class: "list",
                    {entry.responsibilities.iter().map(|desc| rsx! {
                        li { span { "{desc}" } }
                    })}
                }
            })}
        }
    }
}
