use crate::components::data::{ContactLinkHref, CONTACT_LINKS, PROFILE};
use crate::Theme;
use dioxus::prelude::*;

#[component]
pub fn HeaderSection(theme: Signal<Theme>) -> Element {
    rsx! {
        header {
            id: "header",
            div {
                class: "container",
                div {
                    h2 { "{PROFILE.name}" }
                    h4 { "{PROFILE.title}" }
                }
                a {
                    id: "theme-switch",
                    onclick: move |_| theme.set(theme().toggle()),
                    img {
                        src: "{theme().icon_theme()}",
                        alt: "Theme Icon",
                        width: "24",
                    }
                }
            }
            br {}

            p {
                id: "get-in-touch",
                {CONTACT_LINKS.iter().enumerate().map(|(i, link)| {
                    let link_elem = match &link.href {
                        ContactLinkHref::Plain(href) => rsx! {
                            a {
                                href: "{href}",
                                target: link.target.unwrap_or(""),
                                rel: link.rel.unwrap_or(""),
                                download: link.download.unwrap_or(""),
                                "{link.label}"
                            }
                        },
                        ContactLinkHref::ResumeAsset => rsx! {
                            a {
                                // href: asset!("/assets/resume_alexander.pdf"),
                                download: link.download.unwrap_or(""),
                                "{link.label}"
                            }
                        },
                    };

                    if i < CONTACT_LINKS.len() - 1 {
                        rsx! {
                            {link_elem}
                            span { " | " }
                        }
                    } else {
                        link_elem
                    }
                })}
            }
        }
    }
}
