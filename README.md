# NetWatch

This project helps you watch network trafic, filter by protocol, ip using [Tauri](https://tauri.app/) and [Yew](https://yew.rs/). It provides a foundation for building lightweight, secure apps with a Rust backend and a web-based frontend.

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) - Install via rustup
- [Node.js](https://nodejs.org/en/download/) and Yarn - For managing frontend dependencies
- Recommended IDE: [VS Code](https://code.visualstudio.com/) with the [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extensions

## Installation

| Step                  | Command/Instruction                              |
|-----------------------|-------------------------------------------------|
| Clone Repository      | `git clone https://github.com/JhonnyMahony/NetWatch.git` |
| Navigate to Directory | `cd NetWatch`                                   |
| Install Dependencies  | `cargo install tauri-cli` 

## Running the Project

- **Development Mode**: Run `cargo tauri dev` to start the app in development mode with hot reloading.
- **Runtime**: After building, the app runs on Windows, macOS, or Linux, depending on your platform.

## Building the Application

- Run `cargo tauri build` to create a release build.
- Find the executable in the `target/release` directory (e.g., `.exe` for Windows, `.app` for macOS).

## Contributing

Feel free to fork this repository and submit pull requests with improvements or bug fixes. Please follow standard Rust and JavaScript coding conventions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

