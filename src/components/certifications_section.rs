use dioxus::prelude::*;

#[component]
pub fn CertificationsSection() -> Element {
    rsx! {
        section {
            class: "certification",
            h3 { "## Certifications" }
            p {
                a {
                    href: "https://www.credly.com/badges/13918dd1-e5ad-4e81-96c6-95fcb6fb8b3c",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Oracle Certified Associate, Java SE 8 Programmer, Jan 2019"
                }
            }
        }
    }
}
