use dioxus::prelude::*;
use crate::models::employee::{EmployeeRequest, CreateEmployeeRequest};

#[component]
pub fn EmployeeTable() -> Element {
    let mut current_page = use_signal(|| 1i64);
    let mut page_size = use_signal(|| 5i64);
    let mut search_term = use_signal(|| String::new());

    let mut show_modal = use_signal(|| false);
    let mut new_first_name = use_signal(|| String::new());
    let mut new_last_name = use_signal(|| String::new());
    let mut new_email = use_signal(|| String::new());

    // Deine funktionierende Resource-Logik (mit mut)
    let mut employees_resource = use_resource(move || async move {
        let req = EmployeeRequest {
            page: current_page(),
            page_size: page_size(),
            search_term: if search_term().is_empty() { None } else { Some(search_term()) },
        };
        crate::server::get_employees(req).await
    });

    let res_data = employees_resource.read_unchecked();
    
    // Deine funktionierende Pagination-Berechnung
    let (pagination_text, prev_disabled, next_disabled) = if let Some(Ok(res)) = &*res_data {
        let start = (current_page() - 1) * page_size() + 1;
        let end = (start + page_size() - 1).min(res.total_count);
        let text = format!("{}-{} of {}", start, end, res.total_count);
        (text, current_page() <= 1, current_page() * page_size() >= res.total_count)
    } else {
        ("Loading...".to_string(), true, true)
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full relative",
            
            // --- Header: Search & Add Button (Dein ursprüngliches Design) ---
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
                button {
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg shadow-md transition-all flex items-center gap-2",
                    onclick: move |_| show_modal.set(true),
                    "Add Employee"
                }
            }

            // --- Table Content (Dein ursprüngliches Design mit blauem Header) ---
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
                        if let Some(Ok(res)) = &*res_data {
                            if res.employees.is_empty() {
                                tr { td { colspan: "4", class: "px-6 py-8 text-center text-slate-500", "No employees found." } }
                            } else {
                                for emp in &res.employees {
                                    tr { class: "h-16 hover:bg-blue-50 transition-colors", key: "{emp.id}",
                                        td { class: "px-6 py-4 text-sm text-slate-900 truncate", "{emp.id}" }
                                        td { class: "px-6 py-4 text-sm text-slate-700 truncate", "{emp.first_name}" }
                                        td { class: "px-6 py-4 text-sm text-slate-700 truncate", "{emp.last_name}" }
                                        td { class: "px-6 py-4 text-sm text-slate-700 truncate", "{emp.email}" }
                                    }
                                }
                            }
                        } else {
                            // Skeleton Loader
                            for _ in 0..5 {
                                tr { class: "h-16 animate-pulse",
                                    td { colspan: "4", class: "px-6 py-4", div { class: "h-4 bg-slate-100 rounded w-full" } }
                                }
                            }
                        }
                    }
                }
            }

            // --- Pagination Footer (Dein ursprüngliches Design) ---
            div { class: "flex items-center justify-end px-4 py-3 bg-white border border-blue-100 rounded-lg text-sm text-slate-600 shadow-sm min-h-[60px] w-full",
                div { class: "flex items-center justify-end w-48 gap-2",
                    span { "Items per page:" }
                    select {
                        class: "border-b-2 border-blue-500 bg-transparent font-bold text-blue-600 outline-none w-12",
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
                div { class: "w-40 text-center font-mono font-medium text-blue-700", "{pagination_text}" }
                div { class: "flex items-center justify-end w-48 gap-2",
                    button {
                        class: "w-20 px-2 py-1 text-blue-600 font-bold text-right disabled:text-slate-300",
                        disabled: prev_disabled,
                        onclick: move |_| current_page.with_mut(|p| *p -= 1),
                        "PREV"
                    }
                    div { class: "w-[2px] h-4 bg-blue-200" }
                    button {
                        class: "w-20 px-2 py-1 text-blue-600 font-bold text-left disabled:text-slate-300",
                        disabled: next_disabled,
                        onclick: move |_| current_page.with_mut(|p| *p += 1),
                        "NEXT"
                    }
                }
            }

            // --- Modal (Dein ursprüngliches Design mit Formular-Stil) ---
            if show_modal() {
                div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                    div { class: "bg-white rounded-xl shadow-2xl w-[450px] overflow-hidden",
                        div { class: "bg-blue-600 px-6 py-4", 
                            h2 { class: "text-white text-xl font-bold", "Add New Employee" } 
                        }
                        div { class: "p-6 flex flex-col gap-4",
                            input { 
                                class: "border-b-2 p-2 outline-none focus:border-blue-500 transition-colors", 
                                placeholder: "First Name", 
                                oninput: move |e| new_first_name.set(e.value()) 
                            }
                            input { 
                                class: "border-b-2 p-2 outline-none focus:border-blue-500 transition-colors", 
                                placeholder: "Last Name", 
                                oninput: move |e| new_last_name.set(e.value()) 
                            }
                            input { 
                                class: "border-b-2 p-2 outline-none focus:border-blue-500 transition-colors", 
                                placeholder: "Email", 
                                oninput: move |e| new_email.set(e.value()) 
                            }
                        }
                        div { class: "bg-slate-50 px-6 py-4 flex justify-end gap-3",
                            button { 
                                class: "px-4 py-2 text-slate-600 font-bold hover:text-slate-800",
                                onclick: move |_| show_modal.set(false), 
                                "Cancel" 
                            }
                            button {
                                class: "bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded-lg font-bold shadow-md transition-all",
                                onclick: move |_| async move {
                                    let req = CreateEmployeeRequest {
                                        first_name: new_first_name(),
                                        last_name: new_last_name(),
                                        email: new_email(),
                                    };
                                    if let Ok(_) = crate::server::add_employee(req).await {
                                        new_first_name.set(String::new());
                                        new_last_name.set(String::new());
                                        new_email.set(String::new());
                                        show_modal.set(false);
                                        employees_resource.restart();
                                    }
                                },
                                "Save Employee"
                            }
                        }
                    }
                }
            }
        }
    }
}