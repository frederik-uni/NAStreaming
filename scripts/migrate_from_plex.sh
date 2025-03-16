#!/usr/bin/env bash

SCRIPT_DIR="$(dirname "$0")"
if [ ! -d "$SCRIPT_DIR/venv" ]; then
    echo "Virtual environment not found. Creating one..."
    python3 -m venv "$SCRIPT_DIR/venv"
    source "$SCRIPT_DIR/venv/bin/activate"
    pip install --upgrade pip
    pip install plexapi
else
    source "$SCRIPT_DIR/venv/bin/activate"
fi

python3 "$SCRIPT_DIR/migrate_from_plex.py"
