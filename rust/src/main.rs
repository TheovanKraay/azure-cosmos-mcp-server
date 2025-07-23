// Minimal MCP Echo Server
//
// How to run:
// 1. Make sure you have Rust and Cargo installed.
// 2. In this directory, run:
//      cargo run
//
// This will start the MCP server using standard input/output transport.
// You can connect to it using an MCP client or the MCP Inspector tool.

use rmcp::{tool, ServerHandler, ServiceExt};
use std::future::Future;
use rmcp::transport::io;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct EchoService;

impl EchoService {
    #[tool(description = "Echoes back your message wrapped in a custom string")]
    pub async fn echo_message(&self, params: EchoRequest) -> String {
        format!("this is your message: {}", params.message)
    }
}

impl ServerHandler for EchoService {}

#[tokio::main]
async fn main() {
    let service = EchoService;
    // Use stdio transport for MCP
    let _ = service.serve(io::stdio()).await;
}
