use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn css_class(&self) -> &'static str {
        match self {
            Theme::Dark => "gruvbox-dark",
            Theme::Light => "gruvbox-light",
        }
    }

    fn css_file(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/styling/gruvbox-dark.css"),
            Theme::Light => asset!("/assets/styling/gruvbox-light.css"),
        }
    }

    fn toggle_text(&self) -> &'static str {
        match self {
            Theme::Dark => "Jedi",
            Theme::Light => "Sith",
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }
}

#[component]
fn App() -> Element {
    let mut theme = use_signal(|| Theme::Dark);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/index.css")
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/hack.css")
        }
        link {
            rel: "stylesheet",
            href: theme().css_file(),
            id: "theme-css"
        }
        document::Title { "Alexander" }
            
        main {
                class: "{theme().css_class()}",
                id: "top",

                nav {
                    div {
                        class: "nav-links",
                        a { href: "#about", "About" }
                        span { "|" }
                        a { href: "#resume", "Resume" }
                        span { "|" }
                        a { href: "#contact", "Contact" }
                        span { "|" }
                        a {
                            id: "switch",
                            onclick: move |_| theme.set(theme().toggle()),
                            "{theme().toggle_text()}"
                        }
                        span { "|" }
                    }
                }
                
            br {}
            br {}
                
            h2 { "Alexander Alexandrov" }
            h2 { "====================" }
                
            br {}
            br {}
                
            p {
                    "I'm a Sofia based Full Stack Software Engineer, currently working at Proxiad."
                    br {}
                    br {}
                    a {
                        href: "/assets/resume_alexander.pdf",
                        download: true,
                        "Resume ⬇"
                    }
                }

                AboutSection {}
                SkillsSection {}
                CareerSection {}
                EducationSection {}
                CertificationsSection {}
                LanguagesSection {}
                ContactSection {}

                br {}
                a { href: "#top", "Go to top" }
                hr { class: "dashed" }

                footer {
                    img { src: "https://forthebadge.com/images/badges/made-with-rust.svg" }
                    br {}
                    br {}
                    p { "> Copyright 2025 Alexander Alexandrov" }
                    p {
                        a {
                            href: "https://github.com/alexylon/alexandroff.dev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "source code"
                        }
                    }
                }
            }
    }
}

#[component]
fn AboutSection() -> Element {
    rsx! {
        section {
            id: "about-me",
            h3 { id: "about", "## About me" }
            img {
                src: asset!("/assets/images/alex_ascii.jpg"),
                alt: "Alexander Alexandrov"
            }
            p {
                "I am a software developer with a passion for learning and turning creative ideas into practical software solutions. \
                With experience in full-stack engineering, I stay updated on the latest technologies and design patterns. \
                I enjoy solving tech challenges and working in team settings, where I value input and contribute to a positive, communicative atmosphere."
            }
        }
    }
}

#[component]
fn SkillsSection() -> Element {
    rsx! {
        div {
            h3 { id: "resume", "## Skills" }
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
                    span { "Some AWS Services" }
                    span { "++---" }
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
                    span { "Git" }
                    span { "+++--" }
                }
            }
        }
    }
}

#[component]
fn CareerSection() -> Element {
    rsx! {
        div {
            h3 { "## Career History" }
            p { style: "margin-bottom: 0;", "FULL-STACK DEVELOPER" }
            p { i { "Proxiad, Aug 2023 - present" } }

            p { style: "margin-bottom: 0;", "SOFTWARE DEVELOPER" }
            p { i { "DXC Technology, Apr 2020 - Aug 2023" } }

            p { style: "margin-bottom: 0;", "WEB DEVELOPER INTERN" }
            p { i { "Camplight, Sep 2019 - Apr 2020" } }
        }
    }
}

#[component]
fn EducationSection() -> Element {
    rsx! {
        div {
            h3 { "## Education" }
            p { style: "margin-bottom: 0;", "CAMPLIGHT" }
            p { i { "React.js Fundamentals, Apr 2020" } }

            p { style: "margin-bottom: 0;", "SOFIA UNIVERSITY \"ST. KLIMENT OHRIDSKI\"" }
            p { i { "PhD of Theology, 2019" } }

            p { style: "margin-bottom: 0;", "MAXPLUS" }
            p { i { "Java 8 Fundamentals, 2018" } }

            p { style: "margin-bottom: 0;", "UNIVERSITY OF FORESTRY" }
            p { i { "Master of Engineering, 1997" } }

            p { style: "margin-bottom: 0;", "HIGH SCHOOL OF MATHEMATICS" }
            p { i { "Mathematics, 1990" } }
        }
    }
}

#[component]
fn CertificationsSection() -> Element {
    rsx! {
        div {
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

#[component]
fn LanguagesSection() -> Element {
    rsx! {
        div {
            h3 { "## Spoken Languages" }
            ul {
                class: "skill-list",
                li {
                    span { "Bulgarian" }
                    span { "+++++" }
                }
                li {
                    span { "English" }
                    span { "++++-" }
                }
                li {
                    span { "Italian" }
                    span { "+++--" }
                }
                li {
                    span { "Russian" }
                    span { "++---" }
                }
                li {
                    span { "Greek" }
                    span { "+----" }
                }
            }
        }
    }
}

#[component]
fn ContactSection() -> Element {
    rsx! {
        div {
            h3 { id: "contact", "## Get in touch" }
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
