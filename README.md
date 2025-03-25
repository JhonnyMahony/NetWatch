# NetWatch Template

This project helps you watch network trafic, filter by protocol and import export data using [Tauri](https://tauri.app/) and [Yew](https://yew.rs/). It provides a foundation for building lightweight, secure apps with a Rust backend and a web-based frontend.

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
| Install Dependencies  | `cargo install tauri-bundler` and `yarn install`|

## Running the Project

- **Development Mode**: Run `yarn dev` to start the app in development mode with hot reloading.
- **Runtime**: After building, the app runs on Windows, macOS, or Linux, depending on your platform.

## Building the Application

- Run `yarn build` to create a release build.
- Find the executable in the `target/release` directory (e.g., `.exe` for Windows, `.app` for macOS).

## Customizing the Template

- **Frontend**: Modify Yew components in the `src/web` directory.
- **Backend**: Adjust Rust logic in the `src/tauri` directory.
- **Configuration**: Edit `tauri.conf.json` for app settings and `package.json` for scripts.

## Contributing

Feel free to fork this repository and submit pull requests with improvements or bug fixes. Please follow standard Rust and JavaScript coding conventions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

