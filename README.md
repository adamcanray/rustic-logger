# RusticLogger

RusticLogger is a simple logging app built with Rust. It allows users to log messages with different severity levels and store them in a file.

## Features

- Log messages with different severity levels (DEBUG, INFO, WARN, ERROR)
- Store log messages in a file
- Simple and easy to use

## Getting Started

### Installation

Add the following line to your Cargo.toml file:

```
[dependencies]
rustic-logger = "0.1.2"
```

### Usage

Basic usage

```
use rustic_logger::log;

fn main() {
    log(
      "app-log.log",
      "An error occurred",
      None,
      None
    ).unwrap();
}

// [2023-04-01 16:38:40,258] An error occurred
```

With `severity` levels

```
use rustic_logger::log;

fn main() {
    log(
      "app-log.log",
      "Successfully insert data",
      Some(rustic_logger::TSeverity::DEBUG),
      None,
    )
}

// DEBUG [2023-04-02 11:24:45,662] successfully insert data
```

Custom timestamp format by passing `time_format`

```
use rustic_logger::log;

fn main() {
    let filename = "app-log.txt";
    let time_format = Some("%a, %b %d %Y %I:%M:%S %p");
    log(
      filename,
      "Successfully insert new user",
      None,
      time_format
    ).unwrap();

}

// [Sat, Apr 01 2023 04:38:40 PM] Successfully insert new user
```

### API

- > **`log(filename: &'static str, entry: &'static str, severity: Option<TSeverity>, time_format: Option<&'static str>) -> io::Result<String>`**

  This function logs the given entry message to the specified filename with an optional severity and an optional custom time_format timestamp format. The function returns the formatted string.

## Contributing

Contributions are welcome! If you want to contribute to RusticLogger, please fork this repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
