#!/bin/bash

# Start Ollama in the background
ollama serve &

# Wait for Ollama to wake up
sleep 5

# Pull Mistral-Nemo instead of Llama3
if ! ollama list | grep -q "mistral-nemo"; then
  echo "Pulling Mistral-Nemo 12B (Better for French)..."
  ollama pull mistral-nemo
fi

echo "Mistral-Nemo is ready!"
wait