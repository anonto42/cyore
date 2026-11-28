// Command-line options, parsed by hand from std::env::args (no `clap`; see design.md, decision 8).

use std::path::PathBuf;

pub const USAGE: &str = "\
Train a digit classifier on MNIST.

Usage: mnist [OPTIONS]

Options:
  --epochs <N>    Passes over the training set           [default: 10]
  --lr <RATE>     Learning rate                          [default: 0.5]
  --batch <N>     Mini-batch size                        [default: 10]
  --hidden <N>    Hidden-layer neurons                   [default: 128]
  --seed <N>      Random seed (same seed = same run)     [default: 42]
  --data <DIR>    Directory with the four IDX files      [default: data/mnist]
  --help          Show this message

Download the dataset first with `make mnist-data`.";

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub epochs: usize,
    pub learning_rate: f64,
    pub batch_size: usize,
    pub hidden: usize,
    pub seed: u64,
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            epochs: 10,
            learning_rate: 0.5,
            batch_size: 10,
            hidden: 128,
            seed: 42,
            data_dir: PathBuf::from("data/mnist"),
        }
    }
}

/// What the program should do.
#[derive(Debug, PartialEq)]
pub enum Command {
    Train(Config),
    Help,
}

fn positive_int(flag: &str, value: &str) -> Result<usize, String> {
    match value.parse::<usize>() {
        Ok(0) => Err(format!("{flag} must be greater than 0")),
        Ok(n) => Ok(n),
        Err(_) => Err(format!("{flag} expects a whole number, got '{value}'")),
    }
}

fn positive_float(flag: &str, value: &str) -> Result<f64, String> {
    match value.parse::<f64>() {
        Ok(x) if x.is_finite() && x > 0.0 => Ok(x),
        Ok(_) => Err(format!("{flag} must be a positive number, got '{value}'")),
        Err(_) => Err(format!("{flag} expects a number, got '{value}'")),
    }
}

/// Parse the arguments after the program name. Returns an error message for anything invalid.
pub fn parse_args(args: &[String]) -> Result<Command, String> {
    let mut config = Config::default();
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        if flag == "--help" || flag == "-h" {
            return Ok(Command::Help);
        }
        let value = match flag.as_str() {
            "--epochs" | "--lr" | "--batch" | "--hidden" | "--seed" | "--data" => {
                iter.next().ok_or_else(|| format!("{flag} needs a value"))?
            }
            _ => return Err(format!("unknown option '{flag}'")),
        };
        match flag.as_str() {
            "--epochs" => config.epochs = positive_int(flag, value)?,
            "--lr" => config.learning_rate = positive_float(flag, value)?,
            "--batch" => config.batch_size = positive_int(flag, value)?,
            "--hidden" => config.hidden = positive_int(flag, value)?,
            // Seed 0 is allowed: the Rng swaps it for a non-zero constant.
            "--seed" => {
                config.seed = value
                    .parse()
                    .map_err(|_| format!("--seed expects a whole number, got '{value}'"))?
            }
            _ => config.data_dir = PathBuf::from(value), // "--data"
        }
    }
    Ok(Command::Train(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn parse(list: &[&str]) -> Result<Command, String> {
        parse_args(&args(list))
    }

    #[test]
    fn test_defaults() {
        assert_eq!(parse(&[]), Ok(Command::Train(Config::default())));
        let c = Config::default();
        assert_eq!(
            (c.epochs, c.batch_size, c.hidden, c.seed),
            (10, 10, 128, 42)
        );
        assert!((c.learning_rate - 0.5).abs() < 1e-12);
        assert_eq!(c.data_dir, PathBuf::from("data/mnist"));
    }

    #[test]
    fn test_overrides() {
        let cmd = parse(&[
            "--epochs", "1", "--lr", "0.1", "--batch", "32", "--hidden", "64", "--seed", "7",
            "--data", "/tmp/x",
        ]);
        let expected = Config {
            epochs: 1,
            learning_rate: 0.1,
            batch_size: 32,
            hidden: 64,
            seed: 7,
            data_dir: PathBuf::from("/tmp/x"),
        };
        assert_eq!(cmd, Ok(Command::Train(expected)));
    }

    #[test]
    fn test_zero_batch_rejected() {
        let err = parse(&["--batch", "0"]).unwrap_err();
        assert!(err.contains("--batch"));
    }

    #[test]
    fn test_non_numeric_rejected() {
        assert!(parse(&["--epochs", "ten"]).is_err());
        assert!(parse(&["--lr", "fast"]).is_err());
        assert!(parse(&["--lr", "0"]).is_err());
        assert!(parse(&["--lr", "-1"]).is_err());
        assert!(parse(&["--hidden", "0"]).is_err());
    }

    #[test]
    fn test_unknown_flag_rejected() {
        let err = parse(&["--verbose"]).unwrap_err();
        assert!(err.contains("--verbose"));
    }

    #[test]
    fn test_missing_value_rejected() {
        assert!(parse(&["--epochs"]).is_err());
    }

    #[test]
    fn test_help() {
        assert_eq!(parse(&["--help"]), Ok(Command::Help));
        assert_eq!(parse(&["--epochs", "3", "-h"]), Ok(Command::Help));
    }
}
