// Loading MNIST from the IDX binary format.
//
// An IDX file is a big-endian header followed by raw bytes:
//   images: magic 2051, count, rows, cols, then count × rows × cols pixel bytes (0–255)
//   labels: magic 2049, count, then count label bytes (0–9)
// We read the already-decompressed files (`make mnist-data` runs gunzip once).

use std::fmt;
use std::io;
use std::path::Path;

const IMAGE_MAGIC: u32 = 2051;
const LABEL_MAGIC: u32 = 2049;
const IMAGE_HEADER_LEN: usize = 16; // 4 big-endian u32s
const LABEL_HEADER_LEN: usize = 8; // 2 big-endian u32s
const IMAGE_SIDE: usize = 28;
pub const NUM_CLASSES: usize = 10;

pub const TRAIN_IMAGES: &str = "train-images-idx3-ubyte";
pub const TRAIN_LABELS: &str = "train-labels-idx1-ubyte";
pub const TEST_IMAGES: &str = "t10k-images-idx3-ubyte";
pub const TEST_LABELS: &str = "t10k-labels-idx1-ubyte";

/// Everything that can go wrong while loading. Each variant names the file it is about.
#[derive(Debug)]
pub enum DataError {
    /// The file does not exist (usually: the dataset has not been downloaded yet).
    Missing { path: String },
    /// The file exists but could not be read.
    Io { path: String, message: String },
    /// The first 4 bytes are not the expected IDX magic number.
    BadMagic {
        path: String,
        expected: u32,
        actual: u32,
    },
    /// The file is shorter or longer than its header says it should be.
    WrongLength {
        path: String,
        expected: usize,
        actual: usize,
    },
    /// Images are not 28×28.
    BadShape {
        path: String,
        rows: usize,
        cols: usize,
    },
    /// The image file and the label file disagree on how many examples there are.
    CountMismatch {
        images_path: String,
        images: usize,
        labels_path: String,
        labels: usize,
    },
    /// A label byte is not a digit 0–9.
    BadLabel {
        path: String,
        index: usize,
        value: u8,
    },
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::Missing { path } => write!(
                f,
                "{path}: file not found. Download the dataset first with `make mnist-data`."
            ),
            DataError::Io { path, message } => write!(f, "{path}: could not read file: {message}"),
            DataError::BadMagic {
                path,
                expected,
                actual,
            } => write!(
                f,
                "{path}: bad magic number: expected {expected}, found {actual} (wrong or corrupt file?)"
            ),
            DataError::WrongLength {
                path,
                expected,
                actual,
            } if actual < expected => write!(
                f,
                "{path}: file is truncated: header says {expected} bytes, file has {actual}"
            ),
            DataError::WrongLength {
                path,
                expected,
                actual,
            } => write!(
                f,
                "{path}: file is too long: header says {expected} bytes, file has {actual}"
            ),
            DataError::BadShape { path, rows, cols } => write!(
                f,
                "{path}: images are {rows}×{cols}, expected {IMAGE_SIDE}×{IMAGE_SIDE}"
            ),
            DataError::CountMismatch {
                images_path,
                images,
                labels_path,
                labels,
            } => write!(
                f,
                "count mismatch: {images_path} has {images} images but {labels_path} has {labels} labels"
            ),
            DataError::BadLabel { path, index, value } => {
                write!(f, "{path}: label {index} is {value}, expected a digit 0–9")
            }
        }
    }
}

impl std::error::Error for DataError {}

/// A set of labelled images, pixels stored flat: image i is pixels[i * n_inputs ..][..n_inputs].
pub struct Dataset {
    pub n_inputs: usize,
    pub pixels: Vec<f64>,
    pub labels: Vec<u8>,
}

impl Dataset {
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// The i-th image: `n_inputs` values in [0, 1].
    pub fn image(&self, i: usize) -> &[f64] {
        &self.pixels[i * self.n_inputs..(i + 1) * self.n_inputs]
    }

    pub fn label(&self, i: usize) -> usize {
        self.labels[i] as usize
    }
}

/// Read big-endian u32 number `index` from the header. The caller has already checked that
/// `bytes` is long enough, so the slice-to-array conversion cannot fail.
fn header_u32(bytes: &[u8], index: usize) -> u32 {
    let start = index * 4;
    let word: [u8; 4] = bytes[start..start + 4]
        .try_into()
        .expect("caller checked the header length");
    u32::from_be_bytes(word)
}

/// Check the file is at least `header_len` long and starts with `magic`.
fn check_header(path: &str, bytes: &[u8], header_len: usize, magic: u32) -> Result<(), DataError> {
    if bytes.len() < header_len {
        return Err(DataError::WrongLength {
            path: path.to_string(),
            expected: header_len,
            actual: bytes.len(),
        });
    }
    let actual = header_u32(bytes, 0);
    if actual != magic {
        return Err(DataError::BadMagic {
            path: path.to_string(),
            expected: magic,
            actual,
        });
    }
    Ok(())
}

fn check_length(path: &str, bytes: &[u8], expected: usize) -> Result<(), DataError> {
    if bytes.len() != expected {
        return Err(DataError::WrongLength {
            path: path.to_string(),
            expected,
            actual: bytes.len(),
        });
    }
    Ok(())
}

/// Parse an IDX image file. Returns (number of images, pixels scaled to [0, 1]).
/// `path` is only used in error messages.
pub fn parse_images(path: &str, bytes: &[u8]) -> Result<(usize, Vec<f64>), DataError> {
    check_header(path, bytes, IMAGE_HEADER_LEN, IMAGE_MAGIC)?;
    let count = header_u32(bytes, 1) as usize;
    let rows = header_u32(bytes, 2) as usize;
    let cols = header_u32(bytes, 3) as usize;
    if rows != IMAGE_SIDE || cols != IMAGE_SIDE {
        return Err(DataError::BadShape {
            path: path.to_string(),
            rows,
            cols,
        });
    }
    check_length(path, bytes, IMAGE_HEADER_LEN + count * rows * cols)?;

    // Normalise 0–255 to 0.0–1.0 so inputs are on the same scale as the weights.
    let pixels = bytes[IMAGE_HEADER_LEN..]
        .iter()
        .map(|&b| b as f64 / 255.0)
        .collect();
    Ok((count, pixels))
}

/// Parse an IDX label file into one digit per example, in file order.
pub fn parse_labels(path: &str, bytes: &[u8]) -> Result<Vec<u8>, DataError> {
    check_header(path, bytes, LABEL_HEADER_LEN, LABEL_MAGIC)?;
    let count = header_u32(bytes, 1) as usize;
    check_length(path, bytes, LABEL_HEADER_LEN + count)?;

    let labels = &bytes[LABEL_HEADER_LEN..];
    if let Some((index, &value)) = labels
        .iter()
        .enumerate()
        .find(|&(_, &v)| v as usize >= NUM_CLASSES)
    {
        return Err(DataError::BadLabel {
            path: path.to_string(),
            index,
            value,
        });
    }
    Ok(labels.to_vec())
}

/// Combine parsed images and labels, checking that their counts match.
pub fn build_dataset(
    images_path: &str,
    image_bytes: &[u8],
    labels_path: &str,
    label_bytes: &[u8],
) -> Result<Dataset, DataError> {
    let (count, pixels) = parse_images(images_path, image_bytes)?;
    let labels = parse_labels(labels_path, label_bytes)?;
    if count != labels.len() {
        return Err(DataError::CountMismatch {
            images_path: images_path.to_string(),
            images: count,
            labels_path: labels_path.to_string(),
            labels: labels.len(),
        });
    }
    Ok(Dataset {
        n_inputs: IMAGE_SIDE * IMAGE_SIDE,
        pixels,
        labels,
    })
}

fn read_file(path: &Path) -> Result<Vec<u8>, DataError> {
    std::fs::read(path).map_err(|e| {
        let path = path.display().to_string();
        if e.kind() == io::ErrorKind::NotFound {
            DataError::Missing { path }
        } else {
            DataError::Io {
                path,
                message: e.to_string(),
            }
        }
    })
}

fn load_pair(dir: &Path, images_file: &str, labels_file: &str) -> Result<Dataset, DataError> {
    let images_path = dir.join(images_file);
    let labels_path = dir.join(labels_file);
    let image_bytes = read_file(&images_path)?;
    let label_bytes = read_file(&labels_path)?;
    build_dataset(
        &images_path.display().to_string(),
        &image_bytes,
        &labels_path.display().to_string(),
        &label_bytes,
    )
}

/// Load the (training, test) sets from the four decompressed IDX files in `dir`.
pub fn load_mnist(dir: &Path) -> Result<(Dataset, Dataset), DataError> {
    let train = load_pair(dir, TRAIN_IMAGES, TRAIN_LABELS)?;
    let test = load_pair(dir, TEST_IMAGES, TEST_LABELS)?;
    Ok((train, test))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an in-memory IDX image file with the given header values and pixel bytes.
    fn image_file(magic: u32, count: u32, rows: u32, cols: u32, pixels: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for v in [magic, count, rows, cols] {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        bytes.extend_from_slice(pixels);
        bytes
    }

    fn label_file(magic: u32, count: u32, labels: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        for v in [magic, count] {
            bytes.extend_from_slice(&v.to_be_bytes());
        }
        bytes.extend_from_slice(labels);
        bytes
    }

    /// Two 28×28 images: the first all black (0), the second all white (255).
    fn two_images() -> Vec<u8> {
        let mut pixels = vec![0u8; 784];
        pixels.extend(vec![255u8; 784]);
        image_file(2051, 2, 28, 28, &pixels)
    }

    #[test]
    fn test_valid_images() {
        let (count, pixels) = parse_images("img", &two_images()).unwrap();
        assert_eq!(count, 2);
        assert_eq!(pixels.len(), 2 * 784);
        assert!(pixels.iter().all(|p| (0.0..=1.0).contains(p)));
    }

    #[test]
    fn test_black_and_white_pixels() {
        let (_, pixels) = parse_images("img", &two_images()).unwrap();
        assert_eq!(pixels[0], 0.0);
        assert_eq!(pixels[784], 1.0);
    }

    #[test]
    fn test_middle_grey_pixel() {
        let mut pixels = vec![0u8; 784];
        pixels[5] = 51;
        let (_, values) = parse_images("img", &image_file(2051, 1, 28, 28, &pixels)).unwrap();
        assert!((values[5] - 0.2).abs() < 1e-12);
    }

    #[test]
    fn test_wrong_magic() {
        let mut bytes = two_images();
        bytes[3] = 1; // 2051 = 0x0803 → 0x0801 = 2049, a label file's magic
        let err = parse_images("img", &bytes).unwrap_err();
        assert!(matches!(
            err,
            DataError::BadMagic {
                expected: 2051,
                actual: 2049,
                ..
            }
        ));
        let msg = err.to_string();
        assert!(msg.contains("img") && msg.contains("2051") && msg.contains("2049"));
    }

    #[test]
    fn test_truncated_body() {
        let mut bytes = two_images();
        bytes.truncate(bytes.len() - 1);
        let err = parse_images("img", &bytes).unwrap_err();
        assert!(matches!(err, DataError::WrongLength { .. }));
        assert!(err.to_string().contains("truncated"));
    }

    #[test]
    fn test_truncated_header() {
        let err = parse_images("img", &[0, 0, 8]).unwrap_err();
        assert!(matches!(err, DataError::WrongLength { .. }));
    }

    #[test]
    fn test_trailing_bytes_rejected() {
        let mut bytes = two_images();
        bytes.push(0);
        assert!(matches!(
            parse_images("img", &bytes),
            Err(DataError::WrongLength { .. })
        ));
    }

    #[test]
    fn test_wrong_shape() {
        let bytes = image_file(2051, 1, 2, 2, &[0, 0, 0, 0]);
        assert!(matches!(
            parse_images("img", &bytes),
            Err(DataError::BadShape {
                rows: 2,
                cols: 2,
                ..
            })
        ));
    }

    #[test]
    fn test_valid_labels_in_order() {
        let labels = parse_labels("lbl", &label_file(2049, 3, &[7, 0, 9])).unwrap();
        assert_eq!(labels, vec![7, 0, 9]);
    }

    #[test]
    fn test_label_out_of_range() {
        let err = parse_labels("lbl", &label_file(2049, 3, &[1, 10, 2])).unwrap_err();
        assert!(matches!(
            err,
            DataError::BadLabel {
                index: 1,
                value: 10,
                ..
            }
        ));
        let msg = err.to_string();
        assert!(msg.contains("lbl") && msg.contains("10"));
    }

    #[test]
    fn test_count_mismatch() {
        let labels = label_file(2049, 3, &[1, 2, 3]);
        let err = build_dataset("img", &two_images(), "lbl", &labels)
            .err()
            .expect("2 images vs 3 labels must fail");
        assert!(matches!(
            err,
            DataError::CountMismatch {
                images: 2,
                labels: 3,
                ..
            }
        ));
        let msg = err.to_string();
        assert!(msg.contains('2') && msg.contains('3'));
    }

    #[test]
    fn test_dataset_accessors() {
        let labels = label_file(2049, 2, &[4, 8]);
        let data = build_dataset("img", &two_images(), "lbl", &labels).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data.image(1).len(), 784);
        assert_eq!(data.image(1)[0], 1.0);
        assert_eq!(data.label(0), 4);
        assert_eq!(data.label(1), 8);
    }

    #[test]
    fn test_missing_directory_mentions_download() {
        let err = load_mnist(Path::new("/nonexistent/cyore-mnist"))
            .err()
            .expect("missing directory must fail");
        assert!(matches!(err, DataError::Missing { .. }));
        assert!(err.to_string().contains("make mnist-data"));
    }
}
