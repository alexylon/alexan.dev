use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    let start_date = match NaiveDate::from_ymd_opt(2019, 9, 1) {
        Some(date) => date,
        None => return rsx! { div { "" } },
    };

    let current_date = Local::now().date_naive();
    let mut years = current_date.year() - start_date.year();

    if current_date.month() < start_date.month()
        || (current_date.month() == start_date.month() && current_date.day() < start_date.day())
    {
        years -= 1;
    }

    let mut is_image_expanded = use_signal(|| false);

    rsx! {
        section {
            class: "about-section",
            h2 { "About Me" }
            img {
                src: asset!("/assets/images/alex_ascii.jpg"),
                alt: "Alexander Alexandrov",
                style: "cursor: pointer;",
                onclick: move |_| {
                    is_image_expanded.set(true);
                }
            }
            p {
                "Based in Sofia, BG, \
                I'm a software developer who enjoys building reliable web applications and backend systems. \
                My background is in full‑stack development across common web technologies, and I have a strong affinity for Rust when performance and reliability matter. \
                I care about clear naming, thoughtful abstractions, and code that's easy for others to change, and I do my best work on low‑ego, collaborative teams. \
                Always happy to connect and talk about real‑world software design, Rust, and wine."
            }
        }

        if is_image_expanded() {
            div {
                class: "image-overlay",
                style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0, 0, 0, 0.9); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                onclick: move |_| {
                    is_image_expanded.set(false);
                },
                button {
                    class: "close-button",
                    style: "position: absolute; top: 20px; right: 20px; background: rgba(255, 255, 255, 0.9); border: none; border-radius: 50%; width: 40px; height: 40px; font-size: 24px; cursor: pointer; display: flex; align-items: center; justify-content: center; z-index: 1001;",
                    onclick: move |e| {
                        e.stop_propagation();
                        is_image_expanded.set(false);
                    },
                    "×"
                }
                img {
                    src: asset!("/assets/images/alex_ascii.jpg"),
                    alt: "Alexander Alexandrov",
                    style: "max-width: 90%; max-height: 90%; object-fit: contain;",
                    onclick: move |e| {
                        e.stop_propagation();
                    }
                }
            }
        }
    }
}
