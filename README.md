# RusticLogger

RusticLogger is a simple logging app built with Rust. It allows users to log messages with different severity levels and store them in a file.

## Features

- **[On next release]** Log messages with different severity levels (debug, info, warn, error)
- Store log messages in a file
- Simple and easy to use

## Getting Started

### Installation

Add the following line to your Cargo.toml file:

```
[dependencies]
rustic-logger = "0.1.1"
```

### Usage

Basic usage

```
use rustic_logger::log;

fn main() {
    let filename = "app-log.log";
    log(filename, "An error occurred", None).unwrap();

}

// [2023-04-01 16:38:40,258] An error occurred
```

Custom timestamp format by passing `time_format`

```
use rustic_logger::log;

fn main() {
    let filename = "mylog.txt";
    let time_format = Some("%a, %b %d %Y %I:%M:%S %p");
    log(filename, "Successfully insert new user", time_format).unwrap();

}

// [Sat, Apr 01 2023 04:38:40 PM] Successfully insert new user
```

### API

- > **log(filename: &'static str, entry: &'static str, time_format: Option<&'static str>) -> io::Result<String>**

  This function logs the given entry message to the specified filename with an optional custom time_format timestamp format. The function returns the formatted timestamp as a String.

- > **formatted_time_entry(time_format: Option<&'static str>) -> String**

  This function returns a formatted timestamp as a String with an optional custom time_format.

- > **record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()>**

  This function appends the given bytes to the specified filename.

## Contributing

Contributions are welcome! If you want to contribute to RusticLogger, please fork this repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
