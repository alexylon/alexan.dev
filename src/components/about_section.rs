use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    rsx! {
        section {
            id: "about-me",
            h3 { id: "about", "## About Me" }
            img {
                src: asset!("/assets/images/alex_ascii.jpg"),
                alt: "Alexander Alexandrov"
            }
            p {
                "Full-stack developer with 5+ years of experience building scalable applications using Rust, TypeScript, React/Next.js, and Java. Skilled in end-to-end development, delivering clean, reliable code in collaborative environments."
            }
        }
    }
}
