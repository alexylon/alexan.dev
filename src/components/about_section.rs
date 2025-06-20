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

    rsx! {
        section {
            class: "about-me",
            h3 { "## About Me" }
            img {
                src: asset!("/assets/images/alex_ascii.jpg"),
                alt: "Alexander Alexandrov"
            }
            p {
                "Full-stack developer with {years}+ years of experience building scalable applications using Rust, TypeScript, React/Next.js, and Java. Skilled in end-to-end development, delivering clean, reliable code in collaborative environments."
            }
        }
    }
}
