use axum::{
    routing::post,
    Json, Router,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;
use typertask_engine::{parse_script_with_vars, Engine};

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
        true // No token configured, allow open Tailnet access
    }
}

pub async fn execute_handler(
    headers: HeaderMap,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (StatusCode, Json<ErrorResponse>)> {
    if !verify_auth(&headers) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: "error".to_string(),
                message: "Unauthorized: Invalid or missing Bearer token".to_string(),
            }),
        ));
    }

    tracing::info!("Received execution request: {:?}", payload.intent_description);
    
    let commands = match parse_script_with_vars(&payload.script_content, &payload.variables) {
        Ok(cmds) => cmds,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
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
    let app = Router::new()
        .route("/api/v1/execute", post(execute_handler));

    let addr: SocketAddr = bind_addr.parse()?;
    tracing::info!("TyperTask Daemon listening on {} (Auth enabled: {})", addr, std::env::var("TYPERTASK_AUTH_TOKEN").is_ok());
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
