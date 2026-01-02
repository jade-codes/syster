#!/bin/bash
if [ -f "$HOME/.bash_profile" ]; then
    . "$HOME/.bash_profile"
fi
exec bun build src/Viewer.tsx --outdir=dist --target=browser --minify --sourcemap=external
