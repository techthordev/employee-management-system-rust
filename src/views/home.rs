use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[50vh]",
            h1 { class: "text-4xl font-bold text-slate-800", "Welcome to EMS Rust" }
            p { class: "text-slate-500 mt-2", "Select 'View' to manage employees or login to continue." }
        }
    }
}
