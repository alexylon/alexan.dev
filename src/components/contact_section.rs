use dioxus::prelude::*;

#[component]
pub fn ContactSection() -> Element {
    rsx! {
        section {
            id: "contact",
            h3 { "## Get in touch" }
            ul {
                li {
                    a { href: "mailto:contact@alexandrov.cc", "Email" }
                    ":       contact@alexandrov.cc"
                }
                li {
                    a {
                        href: "https://www.google.com/maps/place/Lyulin,+Sofia,+Sofia/@42.7210415,23.2634153,14z",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Location"
                    }
                    ":     Sofia, Bulgaria"
                }
                li {
                    a {
                        href: "https://www.linkedin.com/in/alexandrovalexander/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "LinkedIn"
                    }
                    ":     @alexandrovalexander"
                }
                li {
                    a {
                        href: "https://fosstodon.org/@aal",
                        rel: "me",
                        target: "_blank",
                        "Mastodon"
                    }
                    ":     @aal@fosstodon.org"
                }
                li {
                    a {
                        href: "https://github.com/alexylon",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "GitHub"
                    }
                    ":       alexylon"
                }
                li {
                    a {
                        href: "https://alexandrov.cc",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "React website"
                    }
                    ": alexandrov.cc"
                }
            }
        }
    }
}
