# Window Layout MCP Server

## Supported Operating Systems
* Windows


## Build Instructions
```
cargo build --release
```



## Configuration
### Claude
```json
{
  "mcpServers": {
    "windowLayout": {
      "command": "binary file path",
      "args": []
    }
  }
}
```

## Usage Instructions
* Please arrange all File Explorer windows on my screen
* Please arrange all windows on my screen in a 2x3 grid layout
* Please arrange XX windows from top-left to bottom-right, with each window offset by 128 pixels