use crate::components::data::{ContactLinkHref, CONTACT_LINKS};
use dioxus::prelude::*;

#[component]
pub fn ContactSection() -> Element {
    rsx! {
        section {
            class: "contact",
            {CONTACT_LINKS.iter().enumerate().map(|(i, link)| {
                let link_elem = match &link.href {
                    ContactLinkHref::Plain(href) => rsx! {
                        h4 {a {
                            href: "{href}",
                            target: link.target.unwrap_or(""),
                            rel: link.rel.unwrap_or(""),
                            download: link.download.unwrap_or(""),
                            "{link.label}"
                            }}
                    },

                    ContactLinkHref::ResumeAsset => rsx! {
                        a {
                            // href: asset!("/assets/resume_alexander.pdf"),
                            download: link.download.unwrap_or(""),
                            "{link.label}"
                        }
                    },
                };

                link_elem
            })}
            }
    }
}
