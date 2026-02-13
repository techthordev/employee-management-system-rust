use serde::{Deserialize, Serialize};

// The request parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeRequest {
    pub page: i64,
    pub page_size: i64,
    pub search_term: Option<String>,
}

// The paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeResponse {
    pub employees: Vec<Employee>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Employee {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateEmployeeRequest {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}