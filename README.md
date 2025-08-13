# Folder Tree Generator

A sleek, cross-platform desktop application for visualizing directory structures. Built with a Rust backend and a modern web frontend using Tauri, this tool provides a fast, lightweight, and user-friendly way to generate clean folder trees for documentation, project overviews, or sharing.

## Key Features

  * 🌳 **Instant Visualization:** Select any directory on your system and instantly generate a clean, readable tree structure.
  * ⚙️ **Customizable Ignore List:** Easily exclude unwanted files and folders (like `.git`, `node_modules`, `__pycache__`, etc.) from the output.
  * 📋 **One-Click Copy:** Copy the entire generated tree to your clipboard with a single click for easy pasting into documents, markdown files, or code comments.
  * ⚡ **Fast & Lightweight:** Powered by Rust for high performance and Tauri for a minimal application footprint.
  * 🖥️ **Cross-Platform:** Designed to run natively on Windows, macOS, and Linux.

-----

## Getting Started

To get a local copy up and running for development purposes, follow these simple steps.

### Prerequisites

Ensure you have the following installed on your system:

  * **Rust & Cargo:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
  * **Node.js & npm:** [https://nodejs.org/en/download/](https://nodejs.org/en/download/)
  * **Tauri Prerequisites:** You must also follow the Tauri setup guide for your specific operating system to install necessary dependencies.
      * [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Development Setup

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/rambo1111/folder_tree_generator.git
    ```

2.  **Navigate to the project directory:**

    ```bash
    cd folder-tree-generator
    ```

3.  **Install frontend dependencies:**
    This will install the `@tauri-apps/cli` needed to run the project.

    ```bash
    npm install
    ```

4.  **Run the application in development mode:**
    This command will launch the application with hot-reloading enabled for easy development.

    ```bash
    npm run tauri dev
    ```

-----

## Building for Production

To build the final, distributable application (e.g., an `.msi` installer for Windows), run the following command from the project's root directory:

```bash
npm run tauri build
```

Once the build is complete, you can find the installer/application bundle in the `src-tauri/target/release/bundle/` directory.

-----
