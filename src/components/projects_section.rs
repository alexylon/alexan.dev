use crate::components::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section {
            class: "projects",
            h3 { "## Selected Projects" }
            ul {
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
