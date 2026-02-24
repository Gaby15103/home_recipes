#!/bin/bash

# Start Ollama in the background
ollama serve &



# Check if llama3 is already pulled to avoid redundant downloads
if ! ollama list | grep -q "llama3"; then
  echo "Pulling llama3 model..."
  ollama pull llama3
fi

echo "Llama3 is ready!"
# Bring the background Ollama process to the foreground
wait
