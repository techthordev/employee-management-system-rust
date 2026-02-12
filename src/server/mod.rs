use crate::models::employee::Employee;
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub mod db;

#[server]
pub async fn get_employees() -> Result<Vec<Employee>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        // 1. Database Connection
        let pool = crate::server::db::connect_db()
            .await
            .map_err(|e| ServerFnError::new(format!("DB Connection failed: {}", e)))?;

        // 2. Query
        // We use .map_err() to convert sqlx::Error into something ServerFnError can take
        let employees = sqlx::query_as::<_, Employee>(
            "SELECT id, first_name, last_name, email FROM employee ORDER BY id",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database query failed: {}", e)))?;

        Ok(employees)
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "Server function not available on client",
        ))
    }
}
