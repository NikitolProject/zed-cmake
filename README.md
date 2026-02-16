# zed-cmake

CMake support for [Zed](https://zed.dev) editor.

## Features

- Syntax highlighting via [tree-sitter-cmake](https://github.com/uyha/tree-sitter-cmake)
- LSP integration via [neocmakelsp-fast](https://github.com/NikitolProject/neocmakelsp-fast) (auto-installed)
  - Intelligent code completion
  - Path completions with directory caching
  - Real-time diagnostics and linting
  - Go to definition
  - Hover documentation
  - Signature help
  - Code formatting
  - Rename support
- Build tasks (Configure, Build, Clean, Install, Run, Test)

## Installation

### From Zed Extensions

Search for "CMake" in Zed Extensions and install.

### Dev Extension

```bash
git clone https://github.com/NikitolProject/zed-cmake
# In Zed: Extensions → Install Dev Extension → select zed-cmake folder
```

## Configuration

Add to your `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "cmake": {
      "initialization_options": {
        "format": { "enable": true },
        "lint": { "enable": true },
        "scan_cmake_in_package": false
      }
    }
  }
}
```

## License

MIT
