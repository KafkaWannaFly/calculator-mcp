#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_server = calculator_mcp::init()?;
    http_server.start().await
}