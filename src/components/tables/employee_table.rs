use dioxus::prelude::*;
use crate::models::employee::{EmployeeRequest};

#[component]
pub fn EmployeeTable() -> Element {
    let mut current_page = use_signal(|| 1i64);
    let mut page_size = use_signal(|| 5i64);
    let mut search_term = use_signal(|| String::new());
    
    let employees_resource = use_resource(move || async move {
        let req = EmployeeRequest { 
            page: current_page(),
            page_size: page_size(),
            search_term: if search_term().is_empty() { None } else { Some(search_term()) },
        };
        // Ensure this function exists in src/server/mod.rs and is marked 'pub'
        crate::server::get_employees(req).await
    });

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // --- Search Bar ---
            div { class: "flex justify-between items-center px-2",
                input {
                    class: "border rounded-lg px-4 py-2 w-64 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none",
                    placeholder: "Search employees...",
                    value: "{search_term}",
                    oninput: move |evt| {
                        search_term.set(evt.value());
                        current_page.set(1); 
                    }
                }
            }

            // --- Table Content ---
            div { class: "overflow-x-auto bg-white rounded-lg shadow",
                table { class: "min-w-full table-fixed divide-y divide-slate-200",
                    thead { class: "bg-blue-50",
                        tr {
                            th { class: "w-20 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider", "ID" }
                            th { class: "w-64 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider", "First Name" }
                            th { class: "w-64 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider", "Last Name" }
                            th { class: "px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider", "Email" }
                        }
                    }
                    tbody { class: "divide-y divide-slate-200 bg-white",
                        match &*employees_resource.read_unchecked() {
                            Some(Ok(res)) => rsx! {
                                if res.employees.is_empty() {
                                    tr { td { colspan: "4", class: "px-6 py-8 text-center text-slate-500", "No employees found." } }
                                } else {
                                    for employee in &res.employees {
                                        tr { class: "h-16 hover:bg-blue-50 transition-colors", key: "{employee.id}",
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-900 truncate", "{employee.id}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-700 truncate", "{employee.first_name}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-700 truncate", "{employee.last_name}" }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-slate-700 truncate", "{employee.email}" }
                                        }
                                    }
                                }
                            },
                            _ => rsx! { 
                                // Fixed E0282 by providing clear structure in the skeleton
                                for _ in 0..5 {
                                    tr { class: "h-16 animate-pulse",
                                        td { class: "px-6 py-4", div { class: "h-4 bg-slate-100 rounded" } }
                                        td { class: "px-6 py-4", div { class: "h-4 bg-slate-100 rounded" } }
                                        td { class: "px-6 py-4", div { class: "h-4 bg-slate-100 rounded" } }
                                        td { class: "px-6 py-4", div { class: "h-4 bg-slate-100 rounded" } }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // --- Pagination Bar ---
            div { class: "flex items-center justify-end px-4 py-3 bg-white border border-blue-100 rounded-lg text-sm text-slate-600 shadow-sm min-h-[60px] w-full",
                div { class: "flex items-center justify-end w-48 gap-2",
                    span { "Items per page:" }
                    select {
                        class: "border-b-2 border-blue-500 bg-transparent cursor-pointer outline-none w-12 font-bold text-blue-600",
                        value: "{page_size}",
                        onchange: move |evt| {
                            if let Ok(new_size) = evt.value().parse::<i64>() {
                                page_size.set(new_size);
                                current_page.set(1);
                            }
                        },
                        option { value: "5", "5" }
                        option { value: "10", "10" }
                        option { value: "20", "20" }
                    }
                }

                if let Some(Ok(res)) = &*employees_resource.read_unchecked() {
                    {
                        let start = (current_page() - 1) * page_size() + 1;
                        let end = (start + page_size() - 1).min(res.total_count);
                        rsx! {
                            div { class: "w-40 text-center font-mono font-medium text-blue-700", "{start}-{end} of {res.total_count}" }
                            div { class: "flex items-center justify-end w-48 gap-2",
                                button {
                                    class: "w-20 px-2 py-1 text-blue-600 font-bold text-right disabled:text-slate-300",
                                    disabled: current_page() <= 1,
                                    onclick: move |_| current_page.with_mut(|p| *p -= 1),
                                    "PREV"
                                }
                                div { class: "w-[2px] h-4 bg-blue-200" }
                                button {
                                    class: "w-20 px-2 py-1 text-blue-600 font-bold text-left disabled:text-slate-300",
                                    disabled: current_page() * page_size() >= res.total_count,
                                    onclick: move |_| current_page.with_mut(|p| *p += 1),
                                    "NEXT"
                                }
                            }
                        }
                    }
                } else {
                    div { class: "w-[352px]" } 
                }
            }
        }
    }
}