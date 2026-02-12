use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    // THIS WAS MISSING: 
    // This signal creates a reactive piece of state (boolean) to track if the dialog is open.
    let mut show_login = use_signal(|| false);

    rsx! {
        nav { class: "bg-slate-900 text-white mb-8 shadow-md",
            div { class: "max-w-7xl mx-auto px-4 flex justify-between h-16 items-center",
                
                // Left side: Brand and Nav-Links
                div { class: "flex items-center gap-8",
                    span { class: "text-xl font-bold text-blue-400", "EMS Rust" }
                    
                    div { class: "flex gap-4 items-center",
                        Link { 
                            to: Route::Home {}, 
                            class: "hover:text-blue-300 transition", 
                            "Home" 
                        }
                        
                        // "View" Dropdown
                        div { class: "relative group py-4",
                            span { class: "cursor-pointer hover:text-blue-300 flex items-center gap-1", 
                                "View"
                                span { class: "text-[10px]", "▼" }
                            }
                            // Dropdown Content
                            div { class: "absolute hidden group-hover:block bg-slate-800 min-w-[150px] shadow-xl rounded-b border-t-2 border-blue-500",
                                Link { 
                                    to: Route::EmployeeView {}, 
                                    class: "block px-4 py-2 hover:bg-slate-700 hover:text-blue-300", 
                                    "Employees" 
                                }
                            }
                        }
                    }
                }

                // Right side: Auth
                div {
                    // Changed from Link to button to trigger the modal instead of a page change
                    button { 
                        class: "bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded transition",
                        onclick: move |_| show_login.set(true),
                        "Login"
                    }
                }
            }
        }

        // LOGIN DIALOG
        // This block only renders if show_login is true
        if show_login() {
            div { 
                class: "fixed inset-0 bg-black/60 flex items-center justify-center z-50",
                onclick: move |_| show_login.set(false),

                div { 
                    class: "bg-white text-slate-900 p-8 rounded-lg shadow-2xl max-w-sm w-full",
                    // Prevents closing the dialog when clicking inside the white box
                    onclick: |e| e.stop_propagation(),

                    h2 { class: "text-2xl font-bold mb-4", "Login" }
                    p { class: "mb-6 text-slate-600", "This is your login dialog placeholder." }
                    
                    button { 
                        class: "w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 transition font-medium",
                        onclick: move |_| show_login.set(false),
                        "Close"
                    }
                }
            }
        }

        // This is where the views (Home, EmployeeView) are rendered
        Outlet::<Route> {}
    }
}