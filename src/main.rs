use anyhow::Result;

use tracing_subscriber::EnvFilter;

use rmcp::ServiceExt;
use window_layout_mcp::mcp::WindowLayoutMCP;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("开始启动窗口MCP服务");

    // 创建Service
    let service = WindowLayoutMCP
        .serve(rmcp::transport::stdio())
        .await
        .inspect(|e| tracing::error!("MCP服务监听错误:{:#?}", e))?;

    // 等待服务关闭
    service.waiting().await?;

    Ok(())
}
