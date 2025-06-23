use crate::components::data::SKILL;
use dioxus::prelude::*;
use crate::Theme;

#[component]
pub fn SkillsSection() -> Element {
    rsx! {
        section {
            class: "skills",
            h2 { "Key Tech Skills" }
            {SKILL.iter().map(|cat| rsx! {
                span { 
                    class: "chip", 
                    "{cat.label}"
                }
            })
            }
        }
    }
}
