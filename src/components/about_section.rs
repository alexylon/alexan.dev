use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    rsx! {
        section {
            id: "about-me",
            h3 { id: "about", "## About me" }
            img {
                src: asset!("/assets/images/alex_ascii.jpg"),
                alt: "Alexander Alexandrov"
            }
            p {
                "I am a dedicated software developer with a genuine enthusiasm for learning and turning ideas into practical solutions. With hands-on experience in full-stack development, I make an effort to stay current with modern tools and best practices. I particularly enjoy working with Rust and exploring its capabilities. I like solving technical challenges and collaborating with others, valuing open communication and a positive team spirit."
            }
        }
    }
}
