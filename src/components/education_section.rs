use crate::components::data::EDUCATION;
use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    rsx! {
        section {
            id: "education",
            h3 { "## Education" }
            ul {
                class: "education-list",
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
