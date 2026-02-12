use dioxus::prelude::*;

#[component]
pub fn EmployeeView() -> Element {
    rsx! {
        // Container to center and limit width to 80%
        div { class: "max-w-[80%] mx-auto mt-10",
            div { class: "bg-white p-8 rounded-lg shadow-sm border border-slate-200",
                h1 { class: "text-2xl font-bold text-slate-800 mb-4", "Employee Overview" }
                
                // Your success message
                p { class: "text-slate-600", 
                    "Success! You navigated to the employee view. This container is now centered and limited in width." 
                }
                
                // Future placeholder for your table
                div { class: "mt-6 p-4 border-2 border-dashed border-slate-200 rounded text-slate-400 text-center",
                    "Table functionality will go here."
                }
            }
        }
    }
}