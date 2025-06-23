use std::rc::Rc;
use crate::components::data::{ContactLinkHref, CONTACT_LINKS};
use dioxus::prelude::*;

#[component]
pub fn ContactSection(contact_section: Signal<Option<Rc<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |cx| contact_section.set(Some(cx.data())),
            class: "contact",
            h2 { "Contact" }
            ul {
            {CONTACT_LINKS.iter().enumerate().map(|(i, link)| {
                let link_elem = match &link.href {
                    ContactLinkHref::Plain(href) => rsx! {
                        li {
                            a {
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
}
