#!/bin/bash
if [ -f "$HOME/.bash_profile" ]; then
  . "$HOME/.bash_profile"
fi
exec bun run --hot src/dev-server.ts
