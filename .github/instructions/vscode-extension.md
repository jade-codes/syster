---
applyTo: 'editors/vscode/**/*.{ts,js,json}'
---

# VS Code Extension Development Instructions

## Extension Overview

The Syster VS Code extension provides IDE integration for SysML v2 and KerML languages. It communicates with the syster-lsp Language Server Protocol implementation to provide rich editing features.

## Technology Stack

- **Language:** TypeScript
- **Build System:** npm, tsc (TypeScript compiler)
- **Extension API:** VS Code Extension API
- **LSP Client:** vscode-languageclient package
- **Packaging:** vsce (Visual Studio Code Extension Manager)

## Project Structure

```
editors/vscode/
├── client/
│   └── src/
│       ├── extension.ts      # Main extension entry point
│       ├── client.ts         # LSP client setup
│       └── server-locator.ts # Server path detection
├── package.json              # Extension manifest and configuration
├── tsconfig.json            # TypeScript configuration
└── README.md                # Extension documentation
```

## Development Guidelines

### VS Code Extension Best Practices

1. **Activation Events**
   - Use precise activation events to minimize extension load time
   - Current activation: on SysML/KerML file opening
   - Don't activate on VS Code startup unless absolutely necessary

2. **Configuration**
   - All user-facing settings should be in `package.json` under `contributes.configuration`
   - Use clear setting names with the `sysml.` prefix
   - Provide sensible defaults
   - Document each setting clearly

3. **Language Server Integration**
   - Use `vscode-languageclient` for LSP communication
   - Handle server crashes and restarts gracefully
   - Provide user feedback for server status
   - Support custom server path configuration for development

4. **Error Handling**
   - Catch and log all errors appropriately
   - Show user-friendly error messages for critical failures
   - Don't crash the extension on recoverable errors
   - Use VS Code's output channel for detailed logs

### TypeScript Guidelines

```typescript
// ✅ Good: Use async/await for asynchronous operations
async function startServer(): Promise<void> {
    try {
        await client.start();
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to start LSP: ${error}`);
    }
}

// ✅ Good: Use proper TypeScript types
interface ServerConfig {
    stdlibEnabled: boolean;
    stdlibPath: string | undefined;
}

// ✅ Good: Use VS Code API properly
const config = vscode.workspace.getConfiguration('syster');
const enabled = config.get<boolean>('stdlib.enabled', true);

// ❌ Bad: Don't use 'any' type
const config: any = vscode.workspace.getConfiguration(); // Don't do this

// ❌ Bad: Don't ignore errors silently
client.start().catch(() => {}); // Don't do this
```

### Extension Configuration

Current extension settings (in `package.json`):
- `syster.stdlib.enabled` - Enable/disable standard library loading
- `syster.stdlib.path` - Custom path to standard library files
- `syster.lsp.path` - Custom path to syster-lsp server binary
- `syster.lsp.trace.server` - Trace LSP communication for debugging

### Testing the Extension

```bash
# Install dependencies
cd editors/vscode
npm install

# Compile TypeScript
npm run compile

# Watch mode for development
npm run watch

# Package the extension
npm run package
```

To test locally:
1. Open the `editors/vscode` folder in VS Code
2. Press F5 to launch Extension Development Host
3. Open a `.sysml` or `.kerml` file to activate the extension
4. Check "Output" panel → "Syster LSP" for logs

### Build & Deployment

```bash
# Compile TypeScript
npm run compile

# Package extension (.vsix file)
npm run package

# Install locally for testing
code --install-extension syster-*.vsix
```

### Code Organization

- **client/src/extension.ts** - Main entry point with `activate()` and `deactivate()`
- **client/src/client.ts** - LSP client setup and configuration
- **client/src/server-locator.ts** - Server binary path detection
- Keep extension logic simple - delegate to LSP server
- Configuration management should be centralized
- Use VS Code's workspace configuration API

### Common Tasks

**Adding a new configuration setting:**
1. Add to `contributes.configuration` in `package.json` with `syster.` prefix
2. Document with clear description and default value
3. Read in extension code using `vscode.workspace.getConfiguration('syster')`
4. Pass to LSP server if needed via initialization options

**Adding a new command:**
1. Define in `contributes.commands` in `package.json`
2. Register in `extension.ts` with `vscode.commands.registerCommand()`
3. Add to activation events if needed

**Updating LSP client:**
1. Modify initialization options in `extension.ts`
2. Ensure server supports the new options
3. Test communication between client and server

### Debugging

Enable detailed logs:
```typescript
// In extension.ts, add to client options:
const clientOptions: LanguageClientOptions = {
    outputChannel: window.createOutputChannel('Syster LSP'),
    traceOutputChannel: window.createOutputChannel('Syster LSP Trace'),
};
```

Check logs in:
- Output panel → "Syster LSP" (general logs)
- Output panel → "Syster LSP Trace" (detailed protocol trace)

### Performance Considerations

- Minimize synchronous operations in activation
- Use lazy initialization where possible
- Avoid blocking the main extension thread
- Let the LSP server handle expensive computations

### Security

- Don't bundle sensitive data or credentials
- Validate all user-provided paths before using
- Use VS Code's secure storage API for any secrets
- Don't execute arbitrary user-provided code

### Integration with syster-lsp

The extension communicates with `crates/syster-lsp/` which provides:
- Document synchronization
- Diagnostics (parse errors)
- Code completion
- Go to definition
- Find references
- Hover information
- Symbol outline
- Code formatting
- Rename symbol
- Semantic tokens (syntax highlighting)
- Inlay hints
- Folding ranges
- Selection ranges

Ensure the extension properly:
1. Starts and stops the server lifecycle
2. Handles server crashes/restarts
3. Passes configuration to server
4. Displays diagnostics to user
5. Registers all supported LSP features

## VS Code Extension Manifest

Key sections in `package.json`:
- `engines.vscode` - Minimum VS Code version
- `activationEvents` - When to activate the extension
- `contributes.languages` - Language definitions (SysML, KerML)
- `contributes.grammars` - TextMate grammars for syntax highlighting
- `contributes.configuration` - Extension settings
- `main` - Entry point file

## Related Documentation

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
- [vscode-languageclient](https://github.com/microsoft/vscode-languageserver-node)
