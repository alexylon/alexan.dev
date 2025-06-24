use crate::components::data::EDUCATION;
use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    rsx! {
        section {
            class: "education-section",
            h2 { "Education" }
            ul {
                class: "education-list list-disc",
                {
                    EDUCATION.iter().map(|ed| rsx! {
                        li {
                            span { "{ed.title}" }
                            span { "{ed.institution}" }
                        }
                    })
                }
            }
        }
    }
}
