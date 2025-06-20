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
    let font_url = asset!("/assets/fonts/JetBrainsMono-Regular.woff2");
    let font_css = format!(
        "@font-face {{
            font-family: 'JetBrains Mono';
            src: url('{}') format('woff2');
            font-weight: normal;
            font-style: normal;
            font-display: swap;
        }}
        body {{
            font-family: 'JetBrains Mono', monospace;
        }}",
        font_url
    );

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
        style {
            "{font_css}"
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
