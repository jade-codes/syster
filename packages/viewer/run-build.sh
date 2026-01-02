#!/bin/bash
source ~/.bash_profile
exec bun build src/Viewer.tsx --outdir=dist --target=browser --minify --sourcemap=external
