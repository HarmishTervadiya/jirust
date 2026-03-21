# 🦀 Jirust (CLI Jira Clone)

> A purely CLI-based, database-free Jira alternative built in Rust for minimalist offline task management.

Jirust is a lightning-fast, terminal-driven issue tracker. This V1 release is designed as a REPL (Read-Eval-Print Loop) application that runs entirely in-memory. It leverages core Rust concepts—like robust `enum`s, `struct`s, and `HashMap`s—to provide safe, instantaneous state management without the overhead of external databases, APIs, or graphical interfaces.

## ✨ Features

* **Interactive REPL Interface:** Continuous terminal loop for seamless, distraction-free task management.
* **In-Memory State:** Instantaneous CRUD operations utilizing flat, ID-based data structures.
* **Hierarchical Organization:** * Manage **Users** and **Organizations**.
  * Create **Boards** linked to specific Organizations.
  * Track **Issues** linked to Boards and assign them to Users.
* **Zero Dependencies:** Built utilizing standard Rust libraries for maximum performance and portability.

## 🏗️ Data Architecture

The application state is maintained globally using integer IDs as foreign keys to prevent deep nesting and ownership conflicts.

* **Organizations:** `id`, `name`
* **Users:** `id`, `name`, `email`
* **Boards:** `id`, `name`, `organization_id`
* **Issues:** `id`, `title`, `description`, `status` (Open, InProgress, Done), `board_id`, `assignee_id`

## 🚀 Getting Started

### Prerequisites
Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

1. Clone the repository:
   ```bash
   git clone [https://github.com/yourusername/jirust.git](https://github.com/yourusername/jirust.git)
   cd jirust
Build the project:

Bash
cargo build
Run the application:

Bash
cargo run
Usage
Upon running cargo run, you will be greeted by the main system overview menu. Simply type the number corresponding to the entity you wish to manage (Organizations, Users, Boards, or Issues) and press Enter to navigate the sub-menus and perform CRUD operations.

🗺️ Roadmap
V1: In-memory REPL, core entity CRUD operations. (Current)

V2: Persistent storage (saving/loading state to local JSON/CSV files).

V3: Advanced CLI argument parsing and terminal UI (TUI) enhancements.

📄 License
This project is licensed under the MIT License.


Would you like me to write the `main.rs` scaffolding containing the `AppState` struct, the `Status` enum, and the basic `get_input` helper function so you can start building the main loop?