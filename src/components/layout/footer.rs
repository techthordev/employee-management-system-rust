use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-white border-t border-slate-200 py-6 mt-auto",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-center",
                div { class: "text-sm text-slate-400", "© 2024 EMS Rust" }
            }
        }
    }
}