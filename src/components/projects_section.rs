use std::rc::Rc;
use crate::components::data::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection(projects_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| projects_section.set(Some(cx.data())),
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
