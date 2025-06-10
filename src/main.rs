use dioxus::prelude::*;

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
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| Theme::Dark);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/styling/index.css")
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
            NavSection { theme }
            br {}
            HeaderSection {}
            AboutSection {}
            SkillsSection {}
            CareerSection {}
            EducationSection {}
            CertificationsSection {}
            LanguagesSection {}
            ContactSection {}
            FooterSection {}
            }
    }
}
