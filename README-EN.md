# im-control

Windows IME (Input Method Editor) control tool for querying and switching input
method status via command line.

## Features

| Command | Description |
|---------|-------------|
| `imc get-im` | Get current input method ID (KLID) |
| `imc set-im <klid>` | Switch to specified input method |
| `imc get-status` | Get IME status (on/off) |
| `imc set-status on/off` | Enable/disable IME |

## Installation

```bash
cargo install-opt
```
