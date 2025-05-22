# 窗口排布MCP Server

## 支持操作系统
* Windows


## 构建方式
```
cargo build --release
```



## 配置说明
### Claude
```json
{
  "mcpServers": {
    "windowLayout": {
      "command": "二进制文件路径",
      "args": []
    }
  }
}
```

## 使用说明
* 请将所有资源管理器窗口为我屏幕在屏幕上
* 请将我屏幕上所有的窗口以2行3列的形式排布
* 请将XX窗口从左上角向右下角排布，每个窗口偏移128像素