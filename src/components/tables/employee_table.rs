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

    // DELETE MODAL STATE
    let mut show_delete_modal = use_signal(|| false);
    let mut delete_target_id = use_signal(|| None::<i64>);

    let mut employees_resource = use_resource(move || async move {
        let req = EmployeeRequest {
            page: current_page(),
            page_size: page_size(),
            search_term: if search_term().is_empty() { None } else { Some(search_term()) },
        };
        crate::server::get_employees(req).await
    });

    let res_data = employees_resource.read_unchecked();

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

            // HEADER
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
                    class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg shadow-md transition-all",
                    onclick: move |_| show_modal.set(true),
                    "Add Employee"
                }
            }

            // TABLE
            div { class: "overflow-x-auto bg-white rounded-lg shadow",
                table { class: "min-w-full table-fixed divide-y divide-slate-200",
                    thead { class: "bg-blue-50",
                        tr {
                            th { class: "w-20 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase", "ID" }
                            th { class: "w-64 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase", "First Name" }
                            th { class: "w-64 px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase", "Last Name" }
                            th { class: "px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase", "Email" }
                            th { class: "w-32 px-6 py-3 text-center text-xs font-bold text-blue-700 uppercase", "Actions" }
                        }
                    }
                    tbody { class: "divide-y divide-slate-200 bg-white",

                        if let Some(Ok(res)) = employees_resource.read().as_ref() {
                            if res.employees.is_empty() {
                                tr {
                                    td { colspan: "5", class: "px-6 py-8 text-center text-slate-500",
                                        "No employees found."
                                    }
                                }
                            } else {
                                for emp in &res.employees {
                                    tr { class: "h-16 hover:bg-blue-50 transition-colors", key: "{emp.id}",

                                        td { class: "px-6 py-4 text-sm", "{emp.id}" }
                                        td { class: "px-6 py-4 text-sm", "{emp.first_name}" }
                                        td { class: "px-6 py-4 text-sm", "{emp.last_name}" }
                                        td { class: "px-6 py-4 text-sm", "{emp.email}" }

                                        td { class: "px-6 py-4 text-center",
                                            button {
                                                class: "text-red-500 hover:text-red-700 transition-colors p-2",
                                                onclick: {
                                                    let id = emp.id;
                                                    move |_| {
                                                        delete_target_id.set(Some(id));
                                                        show_delete_modal.set(true);
                                                    }
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            for _ in 0..5 {
                                tr {
                                    td { colspan: "5", class: "px-6 py-4",
                                        div { class: "h-4 bg-slate-100 rounded w-full animate-pulse" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // PAGINATOR (DEINER – UNVERÄNDERT)
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

            // ADD MODAL
            if show_modal() {
                div {
                    class: "fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm",
                    
                    onclick: move |_| show_modal.set(false),
            
                    div {
                        class: "bg-white rounded-2xl shadow-2xl w-[520px] overflow-hidden",
                        onclick: move |e| e.stop_propagation(),
            
                        // HEADER
                        div { class: "px-8 py-5 border-b",
                            h2 { class: "text-lg font-semibold text-slate-800", "Add New Employee" }
                            p { class: "text-sm text-slate-500 mt-1",
                                "Create a new employee record. All fields are required."
                            }
                        }
            
                        // BODY
                        div { class: "px-8 py-6 flex flex-col gap-5",
            
                            // FIRST NAME
                            div { class: "flex flex-col gap-1",
                                label {
                                    class: "text-sm font-medium text-slate-700",
                                    "First name ",
                                    span { class: "text-red-500", "*" }
                                }
                                input {
                                    class: "border rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition",
                                    value: "{new_first_name}",
                                    oninput: move |e| new_first_name.set(e.value())
                                }
                                if new_first_name().trim().is_empty() {
                                    span { class: "text-xs text-red-500",
                                        "First name is required"
                                    }
                                }
                            }
            
                            // LAST NAME
                            div { class: "flex flex-col gap-1",
                                label {
                                    class: "text-sm font-medium text-slate-700",
                                    "Last name ",
                                    span { class: "text-red-500", "*" }
                                }
                                input {
                                    class: "border rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition",
                                    value: "{new_last_name}",
                                    oninput: move |e| new_last_name.set(e.value())
                                }
                                if new_last_name().trim().is_empty() {
                                    span { class: "text-xs text-red-500",
                                        "Last name is required"
                                    }
                                }
                            }
            
                            // EMAIL
                            div { class: "flex flex-col gap-1",
                                label {
                                    class: "text-sm font-medium text-slate-700",
                                    "Email ",
                                    span { class: "text-red-500", "*" }
                                }
                                input {
                                    class: "border rounded-lg px-4 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition",
                                    value: "{new_email}",
                                    oninput: move |e| new_email.set(e.value())
                                }
                                if !new_email().contains('@') {
                                    span { class: "text-xs text-red-500",
                                        "Please enter a valid email address"
                                    }
                                }
                            }
                        }
            
                        // FOOTER
                        div { class: "px-8 py-5 bg-slate-50 flex justify-end gap-3",
            
                            button {
                                class: "px-4 py-2 rounded-lg border border-slate-300 text-slate-600 hover:bg-slate-100 transition",
                                onclick: move |_| show_modal.set(false),
                                "Cancel"
                            }
            
                            button {
                                class: "px-5 py-2 rounded-lg bg-blue-600 text-white font-semibold hover:bg-blue-700 transition shadow-md disabled:opacity-50 disabled:cursor-not-allowed",
                                disabled: new_first_name().trim().is_empty()
                                    || new_last_name().trim().is_empty()
                                    || !new_email().contains('@'),
            
                                onclick: move |_| async move {
                                    let req = CreateEmployeeRequest {
                                        first_name: new_first_name(),
                                        last_name: new_last_name(),
                                        email: new_email(),
                                    };
            
                                    if crate::server::add_employee(req).await.is_ok() {
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

            // DELETE MODAL
            if show_delete_modal() {
                div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                    div { class: "bg-white rounded-xl shadow-2xl w-[420px] overflow-hidden",

                        div { class: "bg-red-600 px-6 py-4",
                            h2 { class: "text-white text-lg font-bold", "Confirm deletion" }
                        }

                        div { class: "p-6 text-slate-700",
                            p { "Do you really want to delete this employee?" }
                            p { class: "text-sm text-slate-500 mt-2", "This action cannot be undone." }
                        }

                        div { class: "bg-slate-50 px-6 py-4 flex justify-end gap-3",
                            button {
                                onclick: move |_| {
                                    show_delete_modal.set(false);
                                    delete_target_id.set(None);
                                },
                                "Cancel"
                            }
                            button {
                                onclick: move |_| async move {
                                    if let Some(id) = delete_target_id() {
                                        if crate::server::delete_employee(id).await.is_ok() {
                                            if let Some(Ok(res)) = employees_resource.read().as_ref() {
                                                let new_total = res.total_count.saturating_sub(1);
                                                let max_page = ((new_total + page_size() - 1) / page_size()).max(1);
                                                if current_page() > max_page {
                                                    current_page.set(max_page);
                                                }
                                            }
                                            employees_resource.restart();
                                        }
                                    }
                                    show_delete_modal.set(false);
                                    delete_target_id.set(None);
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
