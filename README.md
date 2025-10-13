# BEAR LLM AI

BEAR LLM AI is a powerful, locally-hosted, and privacy-focused desktop application designed to serve as a client for various Large Language Model (LLM) providers and platforms. It offers a seamless and feature-rich experience for interacting with multiple AI models in a single, unified interface.

## Features

- **Cross-Platform Desktop App**: Built with Tauri, BEAR LLM AI runs natively on Windows, macOS, and Linux.
- **Ollama LLM Support**: Connect to local LLMs via Ollama.
- **Rich Chat Interface**: Enjoy a full-featured chat experience with Markdown rendering, code syntax highlighting, and KaTeX for mathematical notation.
- **Conversation Management**: Easily create, manage, pin, and switch between multiple conversations.
- **Prompt Library**: Create, save, and manage a library of custom prompts for reuse.
- **Model Management**: A dedicated interface to manage and configure different AI models.
- **Image Support**: Upload and preview images directly within the chat.
- **Privacy-Focused**: Designed to run locally, ensuring your data and conversations remain private.
- **Custom System Messages**: Set and manage custom system messages for conversations.
- **Wide-Screen Mode**: Toggle a wide-screen mode for a more immersive experience.
- **And much more...**: Including features like a proxy indicator, usage counters, and theme support.

## Tech Stack

- **Frontend**:
  - React
  - TypeScript
  - Vite
  - Tailwind CSS
  - Radix UI & Shadcn UI
  - Zustand for state management
  - `pnpm` for package management
- **Backend**:
  - Rust
  - Tauri
  - SeaORM for database interaction
  - Tokio for asynchronous operations

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

- **Node.js**: Make sure you have Node.js installed.
- **pnpm**: This project uses `pnpm` for package management.
  ```sh
  npm install -g pnpm
  ```
- **Rust**: Follow the official instructions to install Rust and Cargo.
  - [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### System Requirements

- **OS**: Windows 10 (version 1809 or later) / macOS 10.15+ / Linux
- **RAM**: 8GB minimum
- **Storage**: 10GB free space
- **CPU**: 4 cores recommended
- **WebView2** (Windows only): Automatically downloaded and installed on first launch if not present
- **Internet Connection**: Required for first Windows launch (to download WebView2 runtime if needed, ~100MB one-time download)

### Installation & Running

1.  **Clone the repository**
    ```sh
    git clone <repository-url>
    cd project-gouda
    ```
2.  **Install dependencies**
    ```sh
    pnpm install
    ```
3.  **Run the development server**
    This command will start the application in development mode with hot-reloading.
    ```sh
    pnpm tauri dev
    ```
4.  **Build the application**
    To build a production-ready executable for your platform, run:
    ```sh
    pnpm tauri build
    ```

## License

This software is distributed under a proprietary license. Please see the `LICENSE` file for more details.

*Note: While some parts of the codebase may reference the MIT license, the project as a whole is governed by the proprietary license found in the `LICENSE` file. All new contributions fall under this proprietary license.*
