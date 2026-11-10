use axum::{
    routing::{get, post},
    Json, Router,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use typertask_engine::{parse_script_with_vars, Engine, ParseError};

#[derive(Deserialize)]
pub struct ExecuteRequest {
    script_content: String,
    intent_description: Option<String>,
    #[serde(default)]
    variables: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct ExecuteResponse {
    status: String,
    message: String,
    commands_executed: usize,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: String,
    message: String,
    line_number: Option<usize>,
    command_index: Option<usize>,
}

#[derive(Clone)]
pub struct AppState {
    execution_lock: Arc<Mutex<()>>,
}

fn verify_auth(headers: &HeaderMap) -> bool {
    if let Ok(expected_token) = std::env::var("TYPERTASK_AUTH_TOKEN") {
        if expected_token.is_empty() {
            return true;
        }
        let auth_header = headers.get("Authorization").and_then(|v| v.to_str().ok());
        if let Some(auth_val) = auth_header {
            if auth_val.starts_with("Bearer ") {
                let token = &auth_val[7..];
                return token == expected_token;
            }
        }
        false
    } else {
        true
    }
}

pub async fn schema_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "TyperTask-Next API",
        "version": "0.1.0",
        "endpoints": {
            "POST /api/v1/execute": {
                "description": "Execute plain-text macro script with optional variables",
                "body": {
                    "script_content": "string",
                    "intent_description": "string (optional)",
                    "variables": "object (optional key-value map)"
                }
            },
            "GET /api/v1/schema": {
                "description": "Returns OpenAPI schema and macro syntax reference"
            }
        },
        "commands": [
            "DELAY <ms>",
            "SPEED <ms>",
            "MOVE <x> <y>",
            "CLICK [x y]",
            "TYPE \"text\"",
            "KEY <VK_CODE>",
            "REPEAT <count> { ... }"
        ]
    }))
}

pub async fn execute_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (StatusCode, Json<ErrorResponse>)> {
    if !verify_auth(&headers) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: "error".to_string(),
                message: "Unauthorized: Invalid or missing Bearer token".to_string(),
                line_number: None,
                command_index: None,
            }),
        ));
    }

    // Acquire execution mutex to serialize macro execution and prevent race conditions
    let _lock = state.execution_lock.lock().await;

    tracing::info!("Received execution request: {:?}", payload.intent_description);
    
    let commands = match parse_script_with_vars(&payload.script_content, &payload.variables) {
        Ok(cmds) => cmds,
        Err(e) => {
            let line_num = match &e {
                ParseError::InvalidSyntax { line, .. } => Some(*line),
                _ => None,
            };
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                    line_number: line_num,
                    command_index: None,
                }),
            ));
        }
    };

    let count = match Engine::execute(&commands).await {
        Ok(c) => c,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                    line_number: None,
                    command_index: None,
                }),
            ));
        }
    };

    Ok(Json(ExecuteResponse {
        status: "success".to_string(),
        message: "Script executed successfully".to_string(),
        commands_executed: count,
    }))
}

pub async fn run_server(bind_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        execution_lock: Arc::new(Mutex::new(())),
    };

    let app = Router::new()
        .route("/api/v1/execute", post(execute_handler))
        .route("/api/v1/schema", get(schema_handler))
        .with_state(state);

    let addr: SocketAddr = bind_addr.parse()?;
    tracing::info!("TyperTask Daemon listening on {} (Auth enabled: {})", addr, std::env::var("TYPERTASK_AUTH_TOKEN").is_ok());
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
