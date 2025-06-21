use crate::components::data::SKILL;
use dioxus::prelude::*;
use crate::Theme;

#[component]
pub fn SkillsSection(theme: Signal<Theme>) -> Element {
    rsx! {
        section {
            class: "skills",
            h2 { "Key Tech Skills" }
            {SKILL.iter().map(|cat| rsx! {
                span { 
                    class: "chip", 
                    style: "background-color: {theme().chip_bg_color()}",
                    "{cat.label}" 
                }
            })
            }
        }
    }
}
