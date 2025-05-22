pub mod window;
pub mod monitor;

use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// 窗口信息
#[derive(Debug, Clone, Serialize, Deserialize,schemars::JsonSchema)]
pub struct WindowInfo {
    /// 窗口句柄
    #[schemars(description = "窗口句柄 数字类型")]
    pub handle: isize,
    /// 窗口标题
    #[schemars(description = "窗口标题 字符串类型")]
    pub title: String,
    /// 窗口宽度
    #[schemars(description = "窗口宽度 数字类型")]
    pub width: i32,
    /// 窗口高度    
    #[schemars(description = "窗口高度 数字类型")]
    pub height: i32,
    /// 窗口位置 X
    #[schemars(description = "窗口位置 X 数字类型")]
    pub x: i32,
    /// 窗口位置 Y
    #[schemars(description = "窗口位置 Y 数字类型")]
    pub y: i32,
    /// 进程名
    #[schemars(description = "进程名 字符串类型 可空")]
    pub process_name: Option<String>,
}

/// 监视器信息
#[derive(Debug, Clone, Serialize, Deserialize,schemars::JsonSchema)]
pub struct MonitorInfo {
    /// 监视器名称
    #[schemars(description = "监视器名称 字符串类型")]
    pub name: String,
    /// 监视器宽度
    #[schemars(description = "监视器宽度 数字类型")]
    pub width: i32,
    /// 监视器高度
    #[schemars(description = "监视器高度 数字类型")]
    pub height: i32,
    /// 监视器位置 X
    #[schemars(description = "监视器位置 X 数字类型")]
    pub x: i32,
    /// 监视器位置 Y
    #[schemars(description = "监视器位置 Y 数字类型")]
    pub y: i32,
    /// 是否为主监视器
    #[schemars(description = "是否为主监视器 布尔类型")]
    pub is_primary: bool,
    /// 工作区宽度（不包括任务栏等）
    #[schemars(description = "工作区宽度（不包括任务栏等） 数字类型")]
    pub work_width: i32,
    /// 工作区高度（不包括任务栏等）
    #[schemars(description = "工作区高度（不包括任务栏等） 数字类型")]
    pub work_height: i32,
    /// 工作区位置 X
    #[schemars(description = "工作区位置 X 数字类型")]
    pub work_x: i32,
    /// 工作区位置 Y
    #[schemars(description = "工作区位置 Y 数字类型")]
    pub work_y: i32,
    /// 当前DPI缩放比例
    #[schemars(description = "当前DPI缩放比例 数字类型")]
    pub dpi_scale: f64,
}