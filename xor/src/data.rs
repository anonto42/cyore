// The XOR training data: 4 examples.

pub fn xor_data() -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    // Each input has 2 numbers ( A and B )
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    // Each target has 1 number: the correct XOR answer
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    (inputs, targets)
}
