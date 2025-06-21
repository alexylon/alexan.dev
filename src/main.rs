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

#[component]
fn App() -> Element {
    let theme = use_signal(|| Theme::Dark);
    let mut top_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

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
            href: "https://fonts.googleapis.com/css2?family=Podkova:wght@400..800&family=Schibsted+Grotesk:ital,wght@0,400..900;1,400..900&display=swap",
            rel: "stylesheet"
        }
        document::Title { "Alexander" }

        main {
            class: "{theme().css_class()}",
            class: "top",
            onmounted: move |cx| top_element.set(Some(cx.data())),
            // NavSection { theme }
            HeaderSection { theme }
            AboutSection {}
            SkillsSection {}
            CareerSection {}
            ProjectsSection {}
            EducationSection {}
            CertificationsSection {}
            LanguagesSection {}
            FooterSection {}
            ScrollToTop { top_element }
        }
    }
}
