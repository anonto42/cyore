// A dense matrix stored flat in row-major order: element (r, c) lives at data[r * cols + c].
// One contiguous Vec is cache-friendly, unlike XOR's Vec<Vec<f64>> (see design.md, decision 3).

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// The r-th row as a slice.
    pub fn row(&self, r: usize) -> &[f64] {
        &self.data[r * self.cols..(r + 1) * self.cols]
    }

    /// out = W x   (out has `rows` entries, x has `cols` entries)
    ///
    /// Used in the forward pass: each output is the dot product of one row with the input.
    pub fn matvec(&self, x: &[f64], out: &mut [f64]) {
        assert_eq!(x.len(), self.cols, "matvec: input length");
        assert_eq!(out.len(), self.rows, "matvec: output length");
        for (r, o) in out.iter_mut().enumerate() {
            *o = self.row(r).iter().zip(x).map(|(w, xi)| w * xi).sum();
        }
    }

    /// out = Wᵀ v   (out has `cols` entries, v has `rows` entries)
    ///
    /// Used in the backward pass to send the error back through the weights:
    /// out[c] = Σ_r W[r][c] · v[r]. We walk W row by row so memory is read in order.
    pub fn matvec_transposed(&self, v: &[f64], out: &mut [f64]) {
        assert_eq!(v.len(), self.rows, "matvec_transposed: input length");
        assert_eq!(out.len(), self.cols, "matvec_transposed: output length");
        out.fill(0.0);
        for (r, &vr) in v.iter().enumerate() {
            for (o, w) in out.iter_mut().zip(self.row(r)) {
                *o += w * vr;
            }
        }
    }

    /// W += a bᵀ   (a has `rows` entries, b has `cols` entries)
    ///
    /// The gradient of a weight matrix is an outer product (δ times the layer's input),
    /// so this is how per-example gradients are summed into a buffer.
    pub fn add_outer(&mut self, a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), self.rows, "add_outer: a length");
        assert_eq!(b.len(), self.cols, "add_outer: b length");
        let cols = self.cols;
        for (r, &ar) in a.iter().enumerate() {
            if ar == 0.0 {
                continue; // a whole row of zeros to add; skip it
            }
            for (w, bc) in self.data[r * cols..(r + 1) * cols].iter_mut().zip(b) {
                *w += ar * bc;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [[1, 2, 3],
    ///  [4, 5, 6]]
    fn example() -> Matrix {
        Matrix {
            rows: 2,
            cols: 3,
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        }
    }

    #[test]
    fn test_matvec() {
        // [1 2 3] · [1, 0, -1] = -2;  [4 5 6] · [1, 0, -1] = -2
        let mut out = [0.0; 2];
        example().matvec(&[1.0, 0.0, -1.0], &mut out);
        assert_eq!(out, [-2.0, -2.0]);

        // [1 2 3] · [1, 1, 1] = 6;  [4 5 6] · [1, 1, 1] = 15
        example().matvec(&[1.0, 1.0, 1.0], &mut out);
        assert_eq!(out, [6.0, 15.0]);
    }

    #[test]
    fn test_matvec_transposed() {
        // Wᵀ [1, 2] = [1·1 + 4·2, 2·1 + 5·2, 3·1 + 6·2] = [9, 12, 15]
        let mut out = [99.0; 3]; // stale values must be overwritten
        example().matvec_transposed(&[1.0, 2.0], &mut out);
        assert_eq!(out, [9.0, 12.0, 15.0]);
    }

    #[test]
    fn test_add_outer() {
        // [1, 2]ᵀ [1, 0, -1] = [[1, 0, -1], [2, 0, -2]], added to the example matrix
        let mut m = example();
        m.add_outer(&[1.0, 2.0], &[1.0, 0.0, -1.0]);
        assert_eq!(m.data, vec![2.0, 2.0, 2.0, 6.0, 5.0, 4.0]);
    }

    #[test]
    fn test_row() {
        assert_eq!(example().row(1), &[4.0, 5.0, 6.0]);
    }
}
