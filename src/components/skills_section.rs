use dioxus::prelude::*;

#[component]
pub fn SkillsSection() -> Element {
    rsx! {
        section {
            id: "resume",
            h3 { "## Skills" }
            p { style: "margin-bottom: -10px;", "PROGRAMMING LANGUAGES" }
            ul {
                class: "skill-list",
                li {
                    span { "Rust" }
                    span { "+++--" }
                }
                li {
                    span { "JavaScript/TypeScript" }
                    span { "++++-" }
                }
                li {
                    span { "Java" }
                    span { "+++--" }
                }
                li {
                    span { "Terraform" }
                    span { "+----" }
                }
            }
            br {}

            p { style: "margin-bottom: -10px;", "OTHER TECHS" }
            ul {
                class: "skill-list",
                li {
                    span { "React" }
                    span { "++++-" }
                }
                li {
                    span { "Redux" }
                    span { "++++-" }
                }
                li {
                    span { "Next.js" }
                    span { "+++--" }
                }
                li {
                    span { "Vercel AI SDK" }
                    span { "+++--" }
                }
                li {
                    span { "AWS (TypeScript SDK)" }
                    span { "+++--" }
                }
                li {
                    span { "WebAssembly (Rust)" }
                    span { "++---" }
                }
                li {
                    span { "PostgreSQL" }
                    span { "+++--" }
                }
                li {
                    span { "macOS/Linux command line" }
                    span { "+++--" }
                }
                li {
                    span { "Docker" }
                    span { "++---" }
                }
                li {
                    span { "Git" }
                    span { "+++--" }
                }
            }
        }
    }
}
