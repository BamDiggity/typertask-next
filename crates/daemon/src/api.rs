use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use typertask_engine::{parse_script, Engine};

#[derive(Deserialize)]
pub struct ExecuteRequest {
    script_content: String,
    intent_description: Option<String>,
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

pub async fn execute_handler(
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    tracing::info!("Received execution request: {:?}", payload.intent_description);
    
    let commands = match parse_script(&payload.script_content) {
        Ok(cmds) => cmds,
        Err(e) => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
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
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
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
    tracing::info!("TyperTask Daemon listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
