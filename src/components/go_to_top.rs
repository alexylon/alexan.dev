use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn GoToTop() -> Element {
    let mut show_button = use_signal(|| false);

    // Use resource for proper cleanup
    let _scroll_handler = use_resource(move || async move {
        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                log::warn!("Could not get window object");
                return None;
            }
        };

        let handle_scroll = {
            let mut show_button = show_button.clone();
            
            move || {
                let scroll_y = web_sys::window()
                    .and_then(|w| w.page_y_offset().ok())
                    .unwrap_or(0.0);

                show_button.set(scroll_y > 150.0);
            }
        };

        let closure =
            wasm_bindgen::closure::Closure::wrap(Box::new(handle_scroll) as Box<dyn FnMut()>);

        if let Err(e) =
            window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
        {
            log::warn!("Failed to add scroll event listener: {:?}", e);
            return None;
        }

        // Initial check
        let initial_scroll = web_sys::window()
            .and_then(|w| w.page_y_offset().ok())
            .unwrap_or(0.0);
        
        show_button.set(initial_scroll > 150.0);

        Some(closure)
    });

    rsx! {
        a {
            id: "go-to-top",
            class: if show_button() { "" } else { "hidden" },
                href: "#top",
                img {
                    src: asset!("/assets/icons/keyboard_arrow_up_light.svg"),
                    alt: "Up Icon",
                }
            }
    }
}
