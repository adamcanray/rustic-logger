# RusticLogger

RusticLogger is a simple logging app built with Rust. It allows users to log messages with different severity levels and store them in a file.

## Features

- Log messages with different severity levels (debug, info, warn, error)
- Store log messages in a file
- Simple and easy to use

## Getting Started

### Installation

To use RusticLogger, you need to have Rust installed on your machine. You can install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can clone this repository and build the app by running the following commands:

```
git clone https://github.com/your-username/rustic-logger.git
cd rustic-logger
cargo build --release
```

### Usage

To use RusticLogger, simply run the following command:

```
cargo run --release -- log_file_path message severity_level
```

Replace `log_file_path` with the path where you want to store your log file, message with the message you want to log, and severity_level with the severity level of the message (debug, info, warn, or error).

For example:

```
cargo run --release -- /path/to/log.txt "This is a debug message"
```

debug
This will log the message "This is a debug message" with severity level "debug" and store it in the file located at `/path/to/log.txt`.

## Contributing

Contributions are welcome! If you want to contribute to RusticLogger, please fork this repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
