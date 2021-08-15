# Popcat clicker
Auto Popcat clicker running in headless Chrome, written in Rust.

![Aisha popcat](https://i.ytimg.com/vi/RfnzURKzNXo/maxresdefault.jpg)

## What
Launch Chrome by pre-specified thread, open [popcat.click](https://popcat.click) and autoclick 25 pps for each browser.

### Why 25 pps?
popcat.click set pps limit to 800 click per 30 seconds, even if the pps is beyond.

The highest valid click is 26 pps, so I set the bot to 25 pps (for erroneous).

By limiting the click, the saved resources can be distributed on the different browser.

## Prerequisted
Download built script from [release](https://github.com/SaltyAom/popcat-rs/releases)

Or run from sources:
- Rust
- Chrome
- Git

### Rust
Use this scripts to install Rust.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Chrome
For graphical computer, please use [Google Chrome](https://www.google.com/chrome) site directly.

For Ubuntu server, use this scripts to install Google Chrome.
```bash
curl https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb -o chrome.deb && ./sudo apt install chrome.deb
```

## Setup and running
1. Clone project
```bash
git clone https://github.com/saltyaom/popcat-rs && cd popcat-rs
```

2. (Optional) Configure settings to launch chrome by n threads or specified if Chrome should be headless (show as GUI) or not.

- Setting thread:
```rust
5 // Number of windows to be opened, should be based on CPU core
6 const THREADS: u8 = 4;
```

- Setting headless:
By default is false.
Server is recommended to set to `true`
```rust
11 let launch_options = LaunchOptionsBuilder::default()
12     .headless(false)
13     .window_size(Some((200,600)))
14     .build()
```

3. Run the clicker
```bash
cargo run --release
```

For more information, feels free to raise an issues.
