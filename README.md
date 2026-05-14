# im-control

[English version](README-EN.md)

Windows IME (输入法编辑器) 控制工具，用于在命令行查询和切换输入法状态。

## 功能

| 命令 | 说明 |
|------|------|
| `imc get-im` | 获取当前输入法 ID (KLID) |
| `imc set-im <klid>` | 切换到指定输入法 |
| `imc get-status` | 获取 IME 状态 (on/off) |
| `imc set-status on/off` | 启用/禁用 IME |

## 安装

```bash
cargo install-opt
```
