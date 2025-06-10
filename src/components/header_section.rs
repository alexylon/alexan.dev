use dioxus::prelude::*;

#[component]
pub fn HeaderSection() -> Element {
    rsx! {
        header {
            h2 { "Alexander Alexandrov" }
            h2 { "====================" }

            br {}
            br {}

            p {
                "Full Stack Software Developer based in Sofia | Currently at Proxiad."
                br {}
                br {}
                a {
                    href: asset!("/assets/resume_alexander.pdf"),
                    download: "resume_alexander_alexandrov.pdf",
                    "Resume ⬇"
                }
            }
        }
    }
}
