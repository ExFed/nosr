#!/bin/bash

set -euf -o pipefail

# Script to generate pronunciation audio files for NOSR
# Uses Python's gTTS library within a virtual environment

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VENV_DIR="$SCRIPT_DIR/.venv"
AUDIO_DIR="$PROJECT_ROOT/assets/audio"

# Create virtual environment if it doesn't exist
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating virtual environment at $VENV_DIR..."
    python3 -m venv "$VENV_DIR"
fi

# Activate virtual environment
source "$VENV_DIR/bin/activate"

# Install dependencies
echo "Installing dependencies..."
pip install -q -r "$SCRIPT_DIR/requirements.txt"

# Create audio directory if it doesn't exist
mkdir -p "$AUDIO_DIR"

# Define pronunciations
declare -A PRONUNCIATIONS=(
    ["nosr_no-sir"]="no sir"
    ["nosr_no-senior"]="no senior"
    ["nosr_nozzer"]="nozzer"
)

# Generate audio files
echo "Generating pronunciation audio files..."
for filename in "${!PRONUNCIATIONS[@]}"; do
    text="${PRONUNCIATIONS[$filename]}"
    output_file="$AUDIO_DIR/$filename.mp3"

    python3 <<EOF
from gtts import gTTS
from pathlib import Path

text = '$text'
output = '$output_file'
print(f'Generating: "{text}"')
tts = gTTS(text, lang='en', slow=False)
tts.save(output)
print(f' -> {output}')
EOF
done

# Deactivate virtual environment
deactivate
