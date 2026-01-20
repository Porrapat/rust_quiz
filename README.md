# 🦀 Rust Quiz

An interactive quiz application to test your Rust programming knowledge!

🌐 **Try it live:**  
https://rustquiz.porrapat.com

## 🙏 Credits & Inspiration
This project is inspired by:
- **Rustfully** – https://www.youtube.com/@Rustfully/posts  
- **Tutorialspoints** – https://www.tutorialspoint.com/rust

## 🏗️ Project Structure

```
rust_quiz/
├── src/                   # Core library
│   ├── lib.rs             # Module exports
│   ├── quiz.rs            # Quiz data structures and question bank
│   └── engine.rs          # Quiz state management and logic
├── cli/                   # Command-line interface
│   └── src/
│       └── main.rs        # CLI implementation
├── web/                   # Web interface
│   ├── src/
│   │   └── main.rs        # Leptos web app
│   ├── public/
│   │   └── style.css      # Custom Rust-themed styling
│   └── index.html         # Entry HTML file
└── Cargo.toml             # Workspace configuration
```

## 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.80+ (2024 edition)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- For web version: [Trunk](https://trunkrs.dev/) for building WebAssembly apps

Install Trunk:
```bash
cargo install --locked trunk
```

## 🚀 Installation

1. **Clone the repository**:
```bash
git clone https://github.com/Porrapat/rust_quiz.git
cd rust_quiz
```

2. **Build the project**:
```bash
cargo build --release
```

## 🎮 Usage

### CLI Version

Run the command-line quiz:

```bash
cargo run --bin cli
```

### Web Version

Run the web interface:

```bash
cd web
trunk serve --open
```

or

```bash
cd web
trunk serve --open --watch ../src
```

This will:
- Build the WebAssembly application
- Start a local development server (default: http://127.0.0.1:8080)
- Automatically open your browser

**For production build**:
```bash
cd web
trunk build --release
```

The built files will be in `web/dist/`.

## 🛠️ Development

### Running Tests

```bash
cargo test
```

## 📦 Technologies Used

### Core
- **Rust 2024 Edition** - Modern Rust language features

### CLI
- **rand** - Random question selection

### Web
- **[Leptos](https://leptos.dev/)** - Reactive web framework for Rust/WebAssembly
- **[Trunk](https://trunkrs.dev/)** - WebAssembly build tool
- **[Bootstrap 5](https://getbootstrap.com/)** - CSS framework
- **[Bootstrap Icons](https://icons.getbootstrap.com/)** - Icon library

## 📝 License

This project is open source and available under the MIT License.

## AI Tools Used

I use multiple AI tools:

- **ChatGPT**  
  Used for ideas, inspiration, conceptual discussions, and parsing information (including extracting text or structure from images or videos when needed).

- **Gemini**  
  Used for double-checking content and generating visual assets such as logos and illustrative images.

- **Claude (via OpenRouter, VSCode + Cline)**  
  Used as the primary coding assistant for drafting and refining Rust code.

All outputs from AI tools were reviewed, tested, and manually integrated by me.

## 🤝 Contributing and project roadmap!

Contributions are welcome! Feel free to:
- Add new quiz questions
- Improve explanations
- Enhance the UI/UX
- Fix bugs
- Add new features

Roadmap
- Add Tauri for Desktop using.
- Make dynamic questions store (JSON and Databases)
- Add more quiz structure. (Fill in the blank, drag and drop, etc.)

## 📧 Contact

Created by [Porrapat](https://github.com/Porrapat)

---

**Happy Learning! 🦀✨**
