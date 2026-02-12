use crate::models::employee::{Employee, EmployeeRequest, EmployeeResponse}; // Added Request/Response
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub mod db;

#[server]
pub async fn get_employees(req: EmployeeRequest) -> Result<EmployeeResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        // 1. Database Connection
        let pool = crate::server::db::connect_db()
            .await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;

        // Calculation of offset
        let offset = (req.page - 1) * req.page_size;
        let search = format!("%{}%", req.search_term.as_deref().unwrap_or_default());
        
        // 2. Query Total Count
        // Fixed: Space between employee and WHERE
        let count_row = sqlx::query!("SELECT COUNT(*) FROM employee WHERE first_name ILIKE $1 OR last_name ILIKE $1 OR email ILIKE $1", search)
            .fetch_one(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?; // Fixed: .to_string()
        
        let total_count = count_row.count.unwrap_or(0);
        
        // 3. Query Paginated Data
        let employees = sqlx::query_as::<_, Employee>(
                    "SELECT id, first_name, last_name, email FROM employee 
                     WHERE first_name ILIKE $1 OR last_name ILIKE $1 OR email ILIKE $1
                     ORDER BY id LIMIT $2 OFFSET $3",
                )
                .bind(&search)
                .bind(req.page_size)
                .bind(offset)
                .fetch_all(&pool)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
        
        Ok(EmployeeResponse {
            employees,
            total_count,
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "Server function not available on client",
        ))
    }
}