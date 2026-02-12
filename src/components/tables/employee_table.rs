use dioxus::prelude::*;

#[component]
pub fn EmployeeTable() -> Element {
    rsx! {
        div { class: "p-4 bg-white border border-slate-200 rounded shadow-sm",
            p { class: "text-slate-600", "Employee Table placeholder - Data will be displayed here." }
        }
    }
}