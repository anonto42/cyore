// How well does the classifier do? Accuracy, plus a confusion matrix showing *which* digits
// get mistaken for which.

use std::fmt;

/// Fraction of predictions that match the label, in [0, 1].
pub fn accuracy(predictions: &[usize], labels: &[usize]) -> f64 {
    assert_eq!(predictions.len(), labels.len(), "accuracy: length mismatch");
    if labels.is_empty() {
        return 0.0;
    }
    let correct = predictions
        .iter()
        .zip(labels)
        .filter(|(p, l)| p == l)
        .count();
    correct as f64 / labels.len() as f64
}

/// counts[true_label][predicted]: row = what the digit really is, column = what we guessed.
/// A perfect classifier has everything on the diagonal.
pub struct ConfusionMatrix {
    pub n_classes: usize,
    counts: Vec<usize>, // flat, row-major
}

impl ConfusionMatrix {
    pub fn new(predictions: &[usize], labels: &[usize], n_classes: usize) -> Self {
        assert_eq!(
            predictions.len(),
            labels.len(),
            "confusion: length mismatch"
        );
        let mut counts = vec![0; n_classes * n_classes];
        for (&p, &l) in predictions.iter().zip(labels) {
            counts[l * n_classes + p] += 1;
        }
        ConfusionMatrix { n_classes, counts }
    }

    pub fn get(&self, true_label: usize, predicted: usize) -> usize {
        self.counts[true_label * self.n_classes + predicted]
    }

    /// Total number of examples counted (sum of all entries).
    pub fn total(&self) -> usize {
        self.counts.iter().sum()
    }
}

impl fmt::Display for ConfusionMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self
            .counts
            .iter()
            .max()
            .unwrap_or(&0)
            .to_string()
            .len()
            .max(3);
        write!(f, "true\\pred")?;
        for p in 0..self.n_classes {
            write!(f, " {p:>width$}")?;
        }
        writeln!(f)?;
        for t in 0..self.n_classes {
            write!(f, "{t:>9}")?;
            for p in 0..self.n_classes {
                write!(f, " {:>width$}", self.get(t, p))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy() {
        assert!((accuracy(&[1, 2, 3, 4], &[1, 2, 0, 4]) - 0.75).abs() < 1e-12);
        assert_eq!(accuracy(&[], &[]), 0.0);
    }

    #[test]
    fn test_confusion_sums_to_count() {
        let preds = [0, 1, 1, 2, 2, 2, 0];
        let labels = [0, 1, 2, 2, 1, 2, 1];
        let cm = ConfusionMatrix::new(&preds, &labels, 3);
        assert_eq!(cm.total(), preds.len());
        assert_eq!(cm.get(1, 2), 1); // one "1" mistaken for a "2"
        assert_eq!(cm.get(2, 2), 2);
    }

    #[test]
    fn test_perfect_classifier_is_diagonal() {
        let labels = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 3];
        let cm = ConfusionMatrix::new(&labels, &labels, 10);
        for t in 0..10 {
            for p in 0..10 {
                if t != p {
                    assert_eq!(cm.get(t, p), 0);
                }
            }
        }
        assert_eq!(cm.get(3, 3), 3);
    }

    #[test]
    fn test_display_has_header_and_rows() {
        let cm = ConfusionMatrix::new(&[0, 1], &[0, 0], 2);
        let text = cm.to_string();
        assert_eq!(text.lines().count(), 3); // header + one line per class
        assert!(text.starts_with("true\\pred"));
    }
}
