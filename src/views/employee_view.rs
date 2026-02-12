use dioxus::prelude::*;

use crate::components::tables::employee_table::EmployeeTable;

#[component]
pub fn EmployeeView() -> Element {
    rsx! {
        // Container to center and limit width to 80%
        div { class: "max-w-[80%] mx-auto mt-10",
            div { class: "container mx-auto p-4",
                        h2 { class: "text-2xl font-bold mb-4 text-blue-500", "Employee Overview" }
                EmployeeTable {  }
            }
        }
    }
}
