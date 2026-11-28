# mnist-dataset Specification

## Purpose
Provides the MNIST handwritten-digit dataset to the project: fetching the files once, and loading the IDX image and label files into validated, normalised examples ready for training and evaluation.

## Requirements

### Requirement: Dataset download
The project SHALL provide a single command that downloads the four MNIST files (training images, training labels, test images, test labels), decompresses them into `data/mnist/`, and skips files that already exist.

#### Scenario: First download
- **WHEN** the download command runs and `data/mnist/` does not contain the files
- **THEN** the four decompressed IDX files exist in `data/mnist/`

#### Scenario: Already downloaded
- **WHEN** the download command runs and all four files already exist
- **THEN** no file is downloaded again and the command succeeds

### Requirement: Dataset kept out of version control
Downloaded dataset files SHALL NOT be tracked by git.

#### Scenario: Status after download
- **WHEN** the dataset has been downloaded
- **THEN** `git status` reports no new files under `data/`

### Requirement: IDX image loading
The loader SHALL read an IDX image file (magic number 2051, big-endian header of count, rows, columns) and SHALL return one example per image with each pixel scaled from 0–255 to the range [0, 1].

#### Scenario: Valid image file
- **WHEN** a well-formed image file with N images of 28×28 pixels is loaded
- **THEN** N examples of 784 values each are returned, every value within [0, 1]

#### Scenario: Black and white pixels
- **WHEN** a pixel byte is 0 or 255
- **THEN** its loaded value is 0.0 or 1.0 respectively

### Requirement: IDX label loading
The loader SHALL read an IDX label file (magic number 2049, big-endian count) and SHALL return one label per entry, each a digit from 0 to 9.

#### Scenario: Valid label file
- **WHEN** a well-formed label file with N labels is loaded
- **THEN** N labels are returned in file order

#### Scenario: Out-of-range label
- **WHEN** a label byte is greater than 9
- **THEN** loading fails with an error naming the file and the offending value

### Requirement: Structural validation
The loader MUST reject files whose magic number is wrong, whose length does not match the header, or whose image count differs from the matching label count, and SHALL report which file failed and why.

#### Scenario: Wrong magic number
- **WHEN** a file given as an image file starts with a magic number other than 2051
- **THEN** loading fails with an error stating the expected and actual magic numbers

#### Scenario: Truncated file
- **WHEN** a file is shorter than its header declares
- **THEN** loading fails with an error instead of returning partial data

#### Scenario: Count mismatch
- **WHEN** an image file and its label file declare different counts
- **THEN** loading fails with an error stating both counts

### Requirement: Missing dataset guidance
When the dataset directory or a required file is missing, the program SHALL exit with a non-zero status and a message telling the user how to download the dataset, without panicking.

#### Scenario: No data directory
- **WHEN** the classifier starts and `data/mnist/` does not exist
- **THEN** it prints a message that includes the download command and exits with a non-zero status
