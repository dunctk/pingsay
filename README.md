# pingsay

A simple command-line tool for macOS that monitors your internet connection and uses the built-in `say` command to notify you of status changes.

## Platform

**macOS only.** This tool uses the `say` and `osascript` commands, which are specific to macOS.

## Prerequisites

You need to have Rust installed on your system. If you don't have it, you can install it from [rust-lang.org](https://www.rust-lang.org/).

## Installation

1.  Clone this repository:
    ```sh
    git clone https://github.com/your-username/pingsay.git
    cd pingsay
    ```

2.  Install the binary:
    ```sh
    cargo install --path .
    ```
    This will install the `pingsay` executable in your Cargo binary directory (usually `~/.cargo/bin/`), making it available from anywhere in your terminal.

## Usage

To start the connection monitor, run the following command:

```sh
pingsay
```

The tool will then check your internet connection every minute. You will hear a voice notification and see a system notification when your connection is lost or restored.

-   "Internet connection lost"
-   "Internet connection restored"

To stop the monitor, press `Ctrl+C`.
