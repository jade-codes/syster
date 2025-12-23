#!/bin/bash
# Script to create GitHub issues for missing LSP features
# Repository: jade-codes/syster
# Run with: bash create-lsp-issues.sh

echo "Creating GitHub issues for missing LSP features..."
echo "Make sure you have the GitHub CLI (gh) installed and authenticated"
echo ""

# Create labels first (ignore errors if they already exist)
echo "Creating labels..."
gh label create "lsp" --color "0E8A16" --description "Language Server Protocol features" 2>/dev/null || true
gh label create "priority:high" --color "D93F0B" --description "High priority" 2>/dev/null || true
gh label create "priority:medium" --color "FBCA04" --description "Medium priority" 2>/dev/null || true
gh label create "priority:low" --color "0052CC" --description "Low priority" 2>/dev/null || true
echo "Labels ready."
echo ""

# High Priority Issues
gh issue create --title "feat(lsp): Implement incremental text synchronization" \
  --label "enhancement,lsp,priority:high" \
  --body "## Description
Currently using FULL sync which re-parses entire file on every change. Switch to \`TextDocumentSyncKind::INCREMENTAL\` for better performance with large files.

## Benefits
- Significant performance improvement for large files
- Reduced CPU usage during editing
- Better user experience

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_synchronization"

gh issue create --title "feat(lsp): Add code actions support" \
  --label "enhancement,lsp,priority:high" \
  --body "## Description
Implement code actions for quick fixes and refactorings.

## Suggested Actions
- Add import for unresolved symbol
- Extract to definition
- Convert usage to typed usage
- Organize imports

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction"

gh issue create --title "feat(lsp): Implement workspace symbols" \
  --label "enhancement,lsp,priority:high" \
  --body "## Description
Add global symbol search across all files in workspace (Ctrl+T in VS Code).

## Implementation Notes
- Already have complete symbol table infrastructure
- Just need to add \`workspace/symbol\` handler
- Filter and return symbols from \`workspace.symbol_table().all_symbols()\`

## Benefits
- Quick navigation to any symbol in project
- Standard LSP feature expected by users

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_symbol"

gh issue create --title "feat(lsp): Add signature help" \
  --label "enhancement,lsp,priority:high" \
  --body "## Description
Show parameter hints when typing function/feature calls.

## Features
- Display parameter names and types
- Highlight current parameter
- Show documentation for parameters

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_signatureHelp"

gh issue create --title "feat(lsp): Implement inlay hints" \
  --label "enhancement,lsp,priority:high" \
  --body "## Description
Show inline type annotations and parameter names.

## Examples
- Type annotations: \`let vehicle = ...\` → \`let vehicle: Vehicle = ...\`
- Parameter names: \`calculate(5, 10)\` → \`calculate(width: 5, height: 10)\`

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_inlayHint"

# Medium Priority Issues
gh issue create --title "feat(lsp): Add document formatting" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Implement auto-formatting for SysML/KerML documents.

## Features
- Format entire document
- Configurable formatting rules
- Consistent indentation and spacing

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_formatting"

gh issue create --title "feat(lsp): Add document range formatting" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Format only selected range of text.

## Benefits
- Format specific sections without touching entire file
- Better control for users

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_rangeFormatting"

gh issue create --title "feat(lsp): Implement folding ranges" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Define foldable regions for code folding (collapse/expand).

## Foldable Regions
- Package bodies
- Classifier bodies
- Feature bodies
- Import blocks
- Comment blocks

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_foldingRange"

gh issue create --title "feat(lsp): Add selection range" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Smart expand/shrink selection based on AST structure.

## Examples
- Select identifier → Select feature → Select definition → Select package

## Benefits
- Better keyboard-driven editing experience
- Structural selection instead of character-based

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_selectionRange"

gh issue create --title "feat(lsp): Implement call hierarchy" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Show callers and callees of functions/actions.

## Features
- Find all calls to a function
- Find all functions called by a function
- Navigate call chains

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_prepareCallHierarchy"

gh issue create --title "feat(lsp): Add type hierarchy" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Show inheritance/specialization tree for types.

## Features
- Show supertypes (specializations)
- Show subtypes (specializers)
- Navigate type hierarchy

## Implementation Notes
- Already tracking specialization relationships in RelationshipGraph

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_prepareTypeHierarchy"

gh issue create --title "feat(lsp): Implement code lens" \
  --label "enhancement,lsp,priority:medium" \
  --body "## Description
Show inline commands above definitions.

## Examples
- \`3 references\` - clickable to show all references
- \`2 implementations\` - for abstract definitions
- \`Run tests\` - for test definitions

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeLens"

# Lower Priority Issues
gh issue create --title "feat(lsp): Add linked editing support" \
  --label "enhancement,lsp,priority:low" \
  --body "## Description
Rename coupled identifiers simultaneously (e.g., opening and closing tags).

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_linkedEditingRange"

gh issue create --title "feat(lsp): Implement document links" \
  --label "enhancement,lsp,priority:low" \
  --body "## Description
Make imports and references clickable.

## Features
- Click import to open imported file
- Click qualified reference to navigate

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentLink"

gh issue create --title "feat(lsp): Add prepare rename validation" \
  --label "enhancement,lsp,priority:low" \
  --body "## Description
Validate rename operation before executing (check if symbol can be renamed).

## Benefits
- Better error messages
- Prevent invalid renames
- Show rename range to user

## References
- LSP Spec: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_prepareRename"

echo ""
echo "✅ All issues created successfully!"
echo "View them at: https://github.com/jade-codes/syster/issues"
