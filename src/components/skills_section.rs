use std::rc::Rc;
use crate::components::data::SKILL;
use dioxus::prelude::*;

#[component]
pub fn SkillsSection(skills_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| skills_section.set(Some(cx.data())),
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
