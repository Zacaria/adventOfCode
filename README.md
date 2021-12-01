# 50 Code challenges 

A way to explore how to use Rust with code challenges

https://adventofcode.com

## Rust install macOS

`brew install rustup`
`rustup-init`
`rustc --version`
`cargo --version`

## VS Code config to Run rust as scripts :

.vscode/launch.json : 

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Run current file",
      "program": "${workspaceFolder}/target/debug/${fileBasenameNoExtension}",
      "args": [],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo",
      "sourceLanguages": ["rust"]
    }
  ]
}
```

.vscode/tasks.json :
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "cargo",
      "type": "shell",
      "command": "cargo build",
      "args": [],
      "group": {
        "kind": "build",
        "isDefault": true
      }
    }
  ]
}


```
