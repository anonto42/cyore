# Rust Engineering & Style Standards

1. **Idiomatic Rust:**
   - Code is formatted with `rustfmt` and passes `cargo clippy --workspace --all-targets -- -D warnings`.
   - Take slices (`&[f64]`) rather than `&Vec<f64>` in function parameters.
   - No `unwrap()`/`expect()` in library code paths that handle user input; return `Result`.
     `unwrap()` is acceptable in tests and on invariants that are documented in a comment.
   - No `unsafe` code.

2. **Numerical Code:**
   - Use `f64` until there is a measured reason to change (int8 quantisation is a Stage 4 topic).
   - Compare floats with a tolerance (`(a - b).abs() < 1e-6`), never `==`, except for exact zeros.
   - Every backward pass must be covered by a test; prefer a numerical gradient check
     (finite differences with ε ≈ 1e-5) for new layers or loss functions.
   - Document which value a derivative expects (e.g. `sigmoid_derivative` takes the *output*).

3. **Teaching Clarity:**
   - This is a learning repository: name things after the math (`delta`, `weights[neuron][input]`)
     and add short comments explaining the chain-rule step being implemented.
   - Keep modules small and single-purpose (activation, loss, layer, network, data, rng).

4. **Testing:**
   - Unit tests live next to the code in `#[cfg(test)] mod tests`.
   - Training tests use a fixed seed so they are deterministic.
