use crate::components::data::CAREER_ENTRIES;
use dioxus::prelude::*;

#[component]
pub fn CareerSection() -> Element {
    rsx! {
        section {
            class: "career",
            h2 { "Career History" }
            {CAREER_ENTRIES.iter().map(|entry| rsx! {
                h3 {
                    "{entry.title}",
                    i { " • {entry.company_and_period}" }
                }
                ul {
                    class: "list-disc",
                    {entry.responsibilities.iter().map(|desc| rsx! {
                        li { span { "{desc}" } }
                    })}
                }
            })}
        }
    }
}
