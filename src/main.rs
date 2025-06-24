use dioxus::prelude::*;
use std::rc::Rc;

mod components;
use components::*;

fn main() {
    LaunchBuilder::new().launch(App);
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

    fn icon_theme(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/light_mode.svg"),
            Theme::Light => asset!("/assets/icons/dark_mode.svg"),
        }
    }

    fn icon_up(&self) -> Asset {
        match self {
            Theme::Dark => asset!("/assets/icons/keyboard_arrow_up_light.svg"),
            Theme::Light => asset!("/assets/icons/keyboard_arrow_up_dark.svg"),
        }
    }
}

fn get_system_theme() -> Theme {
    use web_sys::window;

    if let Some(window) = window() {
        if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
            if let Some(media_query) = media_query {
                if media_query.matches() {
                    return Theme::Dark;
                }
            }
        }
    }

    Theme::Light
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| get_system_theme());
    let mut top_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let skills_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let experience_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let projects_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let contact_section: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/index.css")
        }
        link {
            rel: "stylesheet",
            href: theme().css_file(),
            class: "theme-css"
        }
        link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
        }
        link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Podkova:wght@400..800&family=Montserrat:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet"
        }
        document::Title { "Alexander" }

        main {
            class: "{theme().css_class()}",
            div {
                class: "resume",
                onmounted: move |cx| top_element.set(Some(cx.data())),
                NavSection { theme, skills_section, experience_section, projects_section, contact_section }
                HeaderSection { theme }
                AboutSection {}
                SkillsSection { skills_section }
                ExperienceSection { experience_section }
                ProjectsSection { projects_section }
                EducationSection {}
                CertificationsSection {}
                LanguagesSection {}
                ContactSection { contact_section }
                FooterSection {}
                ScrollToTop { top_element }
            }
        }
    }
}
