use crate::components::data::SKILL_CATEGORIES;
use dioxus::prelude::*;

#[component]
pub fn SkillsSection() -> Element {
    rsx! {
        section {
            class: "skills",
            h2 { "Key Tech Skills" }
            ul {
                class: "skill-list",
                {
                    SKILL_CATEGORIES.iter().map(|cat| rsx! {
                        li {
                            span { "{cat.label}" }
                            span { "{cat.skills}" }
                        }
                    })
                }
            }
        }
    }
}
