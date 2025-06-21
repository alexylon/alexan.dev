use crate::components::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            class: "projects",
            h2 { "Selected Projects" }
            ul {
                class: "list-disc",
                {
                    PROJECTS.iter().map(|project| rsx! {
                        li {
                            a {
                                href: "{project.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{project.name}:"
                            },
                            " {project.description}"
                        }
                    })
                }
            }
        }
    }
}
