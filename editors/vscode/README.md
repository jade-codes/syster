# SysML v2 Language Support for VS Code

Rich language support for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language) files.

## Features

- **Syntax Highlighting** - Basic highlighting for comments and strings
- **Semantic Highlighting** - Rich, context-aware token coloring via LSP
- **Diagnostics** - Real-time error and warning messages
- **Go to Definition** - Navigate to symbol definitions
- **Find References** - Find all references to a symbol
- **Hover Information** - View symbol information on hover
- **Code Completion** - Intelligent code suggestions
- **Document Symbols** - Outline view of file structure
- **Rename Symbol** - Rename symbols across files
- **Semantic Tokens** - Advanced syntax coloring based on semantic analysis

## Installation

The extension includes the `syster-lsp` language server binary and the SysML v2 standard library - no additional setup required!

Simply install the extension and open any `.sysml` or `.kerml` file to get started.

### Custom Language Server Path (Optional)

If you prefer to use a different language server binary, you can specify a custom path:

1. Path specified in settings: `syster.lsp.path`
2. Environment variable: `SYSTER_LSP_PATH`

## Extension Settings

This extension contributes the following settings:

- `syster.lsp.path`: Path to the syster-lsp binary (leave empty to use bundled version)
- `syster.lsp.trace.server`: Trace LSP communication for debugging (`off`, `messages`, `verbose`)
- `syster.stdlib.enabled`: Load SysML standard library (default: `true`)
- `syster.stdlib.path`: Custom path to SysML standard library directory (leave empty to use bundled version)

## Commands

- `SysML: Restart Language Server` - Restart the language server if it crashes or becomes unresponsive

## Usage

1. Open a `.sysml` or `.kerml` file
2. The extension will automatically activate and connect to the language server
3. All language features will be available immediately

## Troubleshooting

### Language server crashes

Use the restart command: `SysML: Restart Language Server`

### Enable detailed logging

Set `"syster.lsp.trace.server": "verbose"` in your settings, then check:
- View → Output → "SysML Language Server"

### Using a custom language server

If you need to use a custom-built language server (e.g., for development):

1. Build from source: `cargo build --release -p syster-lsp`
2. Set the path in settings: `"syster.lsp.path": "/path/to/syster-lsp"`

## Development

To work on this extension:

```bash
cd editors/vscode
npm install
npm run watch  # Start TypeScript compiler in watch mode

# Press F5 in VS Code to launch Extension Development Host
```

## Contributing

See the main repository: [jade-codes/syster](https://github.com/jade-codes/syster)

## License

MIT - See LICENSE.md in the repository root
