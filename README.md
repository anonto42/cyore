# Rust AI PC Agent – Learning Plan

Goal: learn how AI models are built, train your own small model in Rust that turns plain-English requests into terminal commands, connect it to an agent, and ship it all as one tiny program that runs on very minimal PCs.

## Stage 1 – How models are actually built

- Learn the core ideas: tensors, layers, weights, activation functions, loss, gradient descent and backpropagation (3Blue1Brown neural network series).
- Build a tiny neural network by hand in plain Rust (no libraries): a 2-layer network that learns XOR. Write the forward pass, loss and backpropagation yourself.
- Extend it to recognise handwritten digits (MNIST) to see training on real data.
- Learn the main architectures and what each is for: dense networks, CNNs (images), RNNs, and transformers (text). Watch Andrej Karpathy's "Zero to Hero" series for transformers.

## Stage 2 – Build your own model in Rust

- Learn Burn (the Burn Book): tensors, modules, training loop, saving and loading weights.
- Build your dataset: a JSON or CSV file of pairs like "show my files" → `ls -la`, "how much disk space" → `df -h`. Start with 50–100 commands, 20+ phrasings each (a few thousand examples). Generate variations with scripts.
- Start simple: an intent classifier. A small network reads the sentence and picks one command template from your list, plus a slot model that extracts values like file names or folders. This is only about 1–10 MB.
- Then the advanced version: a tiny transformer (seq2seq) that writes the command itself, about 5–50M parameters.
- Train, then test on phrasings it has never seen. Learn about overfitting, train/test splits and accuracy.

## Stage 3 – Connect the model to an agent

- Write the agent in Rust: read your input, run the model, get the command, show it, ask "Run this? (y/n)", then execute it with std::process::Command and show the output.
- Safety: an allowlist of permitted commands, a block list (rm -rf /, mkfs, dd, shutdown), timeouts, and a confidence threshold. If the model is unsure, it asks you instead of guessing.
- Add a loop for multi-step tasks, simple memory of recent commands, and a Tool trait so new tools are easy to add.
- Learn: how agents work (perceive, decide, act, observe), and how big agents use LLMs the same way.

## Stage 4 – Package for minimal resource use

- Quantise the model weights to 8-bit (int8) to cut size and RAM by about 4x.
- Use Burn's CPU backend (ndarray) so no GPU is needed.
- Embed the weights inside the binary with include\_bytes!, so it is one single file.
- Release build settings: opt-level = 3 (or "z" for size), lto = true, codegen-units = 1, strip = true, panic = "abort".
- Load the model once at start-up and keep it in memory; avoid heavy dependencies.
- Target: a 5–30 MB binary using under 50–100 MB of RAM, running on an old laptop or a Raspberry Pi. Cross-compile for Linux, Windows and ARM.

## Suggested resources

- The Rust Book and Rust by Example
- 3Blue1Brown neural network videos
- Andrej Karpathy, Neural Networks: Zero to Hero
- The Burn Book (burn.dev)
- Candle examples on Hugging Face GitHub (for comparison)
