use dioxus::prelude::*;

#[component]
pub fn EmployeeTable() -> Element {
    let employees_resource = use_resource(move || crate::server::get_employees());

    rsx! {
        div { class: "overflow-x-auto bg-white rounded-lg shadow",
            table { class: "min-w-full divide-y divide-slate-200",
                thead { class: "bg-slate-50",
                    tr {
                        th { class: "px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase", "ID" }
                        th { class: "px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase", "First Name" }
                        th { class: "px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase", "Last Name" }
                        th { class: "px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase", "Email" }
                    }
                }
                tbody { class: "divide-y divide-slate-200",
                    match &*employees_resource.read_unchecked() {
                        Some(Ok(list)) => rsx! {
                            if list.is_empty() {
                                tr { td { colspan: "4", class: "px-6 py-4 text-center text-slate-500", "No employees found." } }
                            } else {
                                for emp in list {
                                    tr { class: "hover:bg-slate-50 transition", key: "{emp.id}",
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-900", "{emp.id}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-600", "{emp.first_name}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-600", "{emp.last_name}" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-600", "{emp.email}" }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { 
                            tr { td { colspan: "4", class: "px-6 py-4 text-center text-red-500", "Error loading data: {e}" } } 
                        },
                        None => rsx! { 
                            tr { td { colspan: "4", class: "px-6 py-4 text-center text-slate-500", "Loading employees..." } } 
                        },
                    }
                }
            }
        }
    }
}