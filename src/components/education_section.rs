use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    rsx! {
        section {
            id: "education",
            h3 { "## Education" }
            p { style: "margin-bottom: 0;", "CAMPLIGHT" }
            p { i { "React.js Fundamentals, Apr 2020" } }
            br {}

            p { style: "margin-bottom: 0;", "SOFIA UNIVERSITY \"ST. KLIMENT OHRIDSKI\"" }
            p { i { "PhD of Theology, 2019" } }
            br {}

            p { style: "margin-bottom: 0;", "MAXPLUS" }
            p { i { "Java 8 Fundamentals, 2018" } }
            br {}

            p { style: "margin-bottom: 0;", "UNIVERSITY OF FORESTRY" }
            p { i { "Master of Engineering, 1997" } }
            br {}

            p { style: "margin-bottom: 0;", "HIGH SCHOOL OF MATHEMATICS" }
            p { i { "Mathematics, 1990" } }
            br {}
        }
    }
}
