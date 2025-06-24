use crate::components::data::EXPERIENCE_ENTRIES;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn ExperienceSection(experience_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| experience_section.set(Some(cx.data())),
            class: "experience-section",
            h2 { "Experience" }
            {EXPERIENCE_ENTRIES.iter().map(|entry| rsx! {
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
