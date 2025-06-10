use dioxus::prelude::*;

#[component]
pub fn CareerSection() -> Element {
    rsx! {
        section {
            id: "career",
            h3 { "## Career History" }
            p { style: "margin-bottom: 0;", "FULL-STACK SOFTWARE DEVELOPER" }
            p { i { "Proxiad, Aug 2023 - present" } }
            br {}

            p { style: "margin-bottom: 0;", "FULL-STACK SOFTWARE DEVELOPER" }
            p { i { "DXC Technology, Apr 2020 - Aug 2023" } }
            br {}

            p { style: "margin-bottom: 0;", "WEB DEVELOPER INTERN" }
            p { i { "Camplight, Sep 2019 - Apr 2020" } }
        }
    }
}
