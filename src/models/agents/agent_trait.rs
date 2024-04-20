use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dyanmic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<RouteObject>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}
