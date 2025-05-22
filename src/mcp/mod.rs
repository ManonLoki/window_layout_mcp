use anyhow::Result;
use rmcp::{Error as McpError, ServerHandler, model::*, tool};

use crate::logic::{self, WindowInfo};

/// 窗口排布MCP工具箱
#[derive(Debug, Clone)]
pub struct WindowLayoutMCP;

#[tool(tool_box)]
impl WindowLayoutMCP {
    /// 创建实例
    pub fn new() -> Self {
        Self
    }

    /// 工具 查询所有已经打开的窗口
    #[tool(description = "查询所有已经打开的窗口")]
    pub async fn query_all_windows(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("查询所有已经打开的窗口");

        // 获取窗口列表
        let windows = logic::window::scan_windows();

        let result = CallToolResult::success(vec![
            Content::text("当前机器已经打开的窗口如下:"),
            Content::json(windows)?,
        ]);

        Ok(result)
    }

    #[tool(description = "获取当前桌面上所有的显示器信息")]
    pub async fn get_monitors(&self) -> Result<CallToolResult, McpError> {
        // 获取显示器列表
        let monitors = logic::monitor::get_monitor_info();

        let result = CallToolResult::success(vec![
            Content::text("当前机器的显示器信息如下:"),
            Content::json(monitors)?,
        ]);
        Ok(result)
    }

    #[tool(description = r#"
        获取窗口布局说明,所有排布进行之前都需要参照此说明,除非用户对排布有特殊需求时可依据用户需求进行调整
    "#)]
    pub async fn get_window_layout_description(&self) -> Result<CallToolResult, McpError> {
        let result =
            CallToolResult::success(vec![
                        Content::text(r#"
                        每次窗口排布前,需要先查询当前的显示器列表,窗口列表。
                        在排布时,需要计算DPI缩放,在某些情况下,也需要计算显示器工作区的大小,
                        在用户未指定进程名或窗口族群时，需要过滤掉可能是系统进程窗口或者并未显示的窗口
                        可以适当的调整窗口的大小,位置,来达到更好的排布效果,当然尽量不要修改用户窗口的大小
                        当存在多个屏幕时,要么用户指定屏幕 如第一块 主屏幕 第二块 其他屏幕等,否则就是所有屏幕累加考虑
                        当排布时,需要考虑窗口的间距,以及窗口的布局方式,如水平布局,垂直布局,网格布局,等
                        在排布窗口时,每排布一个窗口,就需要对当前窗口进行一次置前操作,以保证这个窗口可以正常的显示在屏幕中.
                        当排布计算超出所有屏幕边界时,回归原点重新计算坐标
                        如果十分无法理解用户的诉求，请及时询问用户，以保证排布的准确性
                        "#.to_string())]);
        Ok(result)
    }

    #[tool(description = r#"
       设置窗口位置,大小等操作
    "#)]
    pub async fn set_window_position(
        &self,
        #[tool(aggr)] window: WindowInfo,
    ) -> Result<CallToolResult, McpError> {
        match logic::window::set_window_position(window) {
            Ok(_) => {
                let result =
                    CallToolResult::success(vec![Content::text("设置窗口位置成功".to_string())]);
                Ok(result)
            }
            Err(e) => Err(McpError::new(
                ErrorCode(500),
                "设置窗口位置失败",
                Some(serde_json::to_value(e.to_string()).expect("转换错误失败")),
            )),
        }
    }

    #[tool(description = r#"将窗口置前"#)]
    pub async fn set_window_to_top(
        &self,
        #[tool(param)]
        #[schemars(description = "窗口句柄")]
        handle: isize,
    ) -> Result<CallToolResult, McpError> {
        match logic::window::set_window_to_top(handle) {
            Ok(_) => {
                let result =
                    CallToolResult::success(vec![Content::text("窗口置前成功".to_string())]);
                Ok(result)
            }
            Err(e) => Err(McpError::new(
                ErrorCode(500),
                "窗口置前失败",
                Some(serde_json::to_value(e.to_string()).expect("转换错误失败")),
            )),
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for WindowLayoutMCP {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "我可以完成对当前操作系统桌面上所有窗口的查询和排布"
                    .to_string(),
            ),
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
        }
    }
}

impl Default for WindowLayoutMCP {
    fn default() -> Self {
        Self::new()
    }
}
