use dioxus::prelude::*;
use crate::models::employee::{Employee, EmployeeRequest, EmployeeResponse, CreateEmployeeRequest};

#[cfg(feature = "server")]
pub mod db;

#[server]
pub async fn get_employees(req: EmployeeRequest) -> Result<EmployeeResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = db::connect_db().await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;

        let offset = (req.page - 1) * req.page_size;
        let search = format!("%{}%", req.search_term.unwrap_or_default());

        let employees = sqlx::query_as!(
            Employee,
            r#"
            SELECT id, first_name, last_name, email 
            FROM employee 
            WHERE first_name ILIKE $1 OR last_name ILIKE $1 OR email ILIKE $1
            ORDER BY id ASC
            LIMIT $2 OFFSET $3
            "#,
            search,
            req.page_size,
            offset
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        let total_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM employee WHERE first_name ILIKE $1 OR last_name ILIKE $1 OR email ILIKE $1",
            search
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or(0);

        Ok(EmployeeResponse {
            employees,
            total_count,
        })
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server function not available on client"))
    }
}

#[server]
pub async fn add_employee(req: CreateEmployeeRequest) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let pool = db::connect_db().await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;
        
        sqlx::query!(
            "INSERT INTO employee (first_name, last_name, email) VALUES ($1, $2, $3)",
            req.first_name,
            req.last_name,
            req.email
        )
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server function not available on client"))
    }
}

#[server]
pub async fn delete_employee(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use sqlx::Error as SqlxError;
        let pool = db::connect_db().await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;
        
        sqlx::query!("DELETE FROM employee WHERE id = $1", id)
            .execute(&pool)
            .await
            .map_err(|e: SqlxError| ServerFnError::new(format!("Database error: {}", e)))?;

        Ok(())
    }
    #[cfg(not(feature = "server"))]
    { Err(ServerFnError::new("Server function not available on client")) }
}