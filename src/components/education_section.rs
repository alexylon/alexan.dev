use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    rsx! {
        section {
            id: "education",
            h3 { "## Education" }
            p { style: "margin-bottom: 0;", "CAMPLIGHT" }
            p { i { "React and JavaScript/TypeScript Fundamentals" } }
            br {}
            
            p { style: "margin-bottom: 0;", "MAXPLUS" }
            p { i { "Java Fundamentals" } }
            br {}

            p { style: "margin-bottom: 0;", "SOFIA UNIVERSITY \"ST. KLIMENT OHRIDSKI\"" }
            p { i { "PhD of Theology" } }
            br {}

            p { style: "margin-bottom: 0;", "UNIVERSITY OF FORESTRY" }
            p { i { "Master of Engineering" } }
            br {}

            p { style: "margin-bottom: 0;", "HIGH SCHOOL OF MATHEMATICS" }
            p { i { "Mathematics" } }
            br {}
        }
    }
}
