<!-- <p align="center">
    <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" align="center" width="30%">
</p> -->
<p align="center"><h1 align="center">TFJINJA</h1></p>
<p align="center">
	<em>Use Jinja templates with Terraform</em>
</p>
<p align="center">
	<!-- local repository, no metadata badges. --></p>
<p align="center">Built with the tools and technologies:</p>
<p align="center">
	<img src="https://img.shields.io/badge/Rust-000000.svg?style=default&logo=Rust&logoColor=white" alt="Rust">
</p>
<br>

##  Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
  - [Project Index](#project-index)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Testing](#testing)
- [Project Roadmap](#project-roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

---

##  Overview

tfjinja is a specialized command-line tool that streamlines Terraform infrastructure management by enabling Jinja-style templating in Terraform files. It helps DevOps teams reduce code duplication and maintain consistency by allowing dynamic content generation, parameter substitution, and template reuse across infrastructure configurations. Perfect for organizations managing complex, multi-environment Terraform deployments.

---

##  Features

|      | Feature         | Summary       |
| :--- | :---:           | :---          |
| ⚙️  | **Architecture**  | <ul><li>Built in `Rust` with a modular design focusing on template processing</li><li>Core components split into `main.rs`, `process_file.rs`, and `render_jinja.rs`</li><li>Follows Terraform-compatible template processing architecture</li></ul> |
| 🔩 | **Code Quality**  | <ul><li>Implements Rust 2021 edition standards</li><li>Clear separation of concerns between file processing and template rendering</li><li>Structured error handling throughout the codebase</li></ul> |
| 📄 | **Documentation** | <ul><li>Configuration documentation in `settings.toml`</li><li>Example templates provided in `examples/` directory</li><li>Clear package manifest documentation in `Cargo.toml`</li></ul> |
| 🔌 | **Integrations**  | <ul><li>Terraform template integration via `.tftpl` files</li><li>MySQL socket connectivity support through container templates</li><li>Kubernetes deployment integration capabilities</li></ul> |
| 🧩 | **Modularity**    | <ul><li>Separate modules for file processing and template rendering</li><li>Configurable template extensions and prefix identifiers</li><li>Pluggable template processing architecture</li></ul> |
| 🧪 | **Testing**       | <ul><li>Supports testing via `cargo test`</li><li>Example templates for validation</li></ul> |
| ⚡️  | **Performance**   | <ul><li>Efficient file system traversal using `walkdir`</li><li>Optimized template processing with `tera` engine</li><li>Recursive directory scanning capabilities</li></ul> |
| 🛡️ | **Security**      | <ul><li>Managed security contexts for container templates</li><li>Controlled file system access patterns</li></ul> |
| 📦 | **Dependencies**  | <ul><li>`clap` for CLI argument parsing</li><li>`tera` for template processing</li><li>`config` for configuration management</li><li>`walkdir` for filesystem operations</li></ul> |

---

##  Project Structure

```sh
└── tfjinja/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── examples
    │   └── socat_container.tftpl
    ├── readme-ai.md
    ├── settings.toml
    ├── src
    │   ├── main.rs
    │   ├── process_file.rs
    │   └── render_jinja.rs
    └── tests
        ├── inside
        ├── test.tf
        └── test_two.tf
```


###  Project Index
<details open>
	<details> <!-- __root__ Submodule -->
		<summary><b>__root__</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
				<td>- Project manifest configuration defining core dependencies and metadata for tfjinja, a Rust-based application<br>- Specifies essential crates including clap for command-line argument parsing, walkdir for filesystem traversal, config for configuration management, and tera for template processing<br>- Sets project version to 0.1.0 and targets Rust 2021 edition standards.</td>
			</tr>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/settings.toml'>settings.toml</a></b></td>
				<td>- Configuration settings for Terraform template processing define core parameters used throughout the project, including the target file extension and template prefix identifier<br>- These settings establish standardized markers for template recognition and processing, enabling consistent handling of Terraform-specific template files across the entire infrastructure codebase.</td>
			</tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- examples Submodule -->
		<summary><b>examples</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/examples\socat_container.tftpl'>socat_container.tftpl</a></b></td>
				<td>- Defines a sidecar container template for establishing MySQL socket connectivity in Kubernetes deployments<br>- The socat container facilitates communication between applications requiring Unix socket connections and MySQL services running over TCP, while managing resource allocations and security contexts<br>- Essential for legacy applications needing Unix socket-based database connections in containerized environments.</td>
			</tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- src Submodule -->
		<summary><b>src</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/src\main.rs'>main.rs</a></b></td>
				<td>- Serves as the entry point for a template processing utility that recursively scans directories for files with specified extensions<br>- Manages command-line arguments and configuration settings to identify template files, processes them using Jinja-style templating, and handles file operations<br>- Coordinates between different modules to enable template rendering and file processing functionality.</td>
			</tr>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/src\process_file.rs'>process_file.rs</a></b></td>
				<td>- Processes and manages file content transformations by handling template inclusions and parameter substitutions<br>- Supports dynamic file content injection with customizable indentation while maintaining the original file structure<br>- Enables template rendering with variable parameters and provides mechanisms for external file inclusion, making it a core component for file-based template processing operations.</td>
			</tr>
			<tr>
				<td><b><a href='C:\Private\20-RUST\tfjinja/blob/master/src\render_jinja.rs'>render_jinja.rs</a></b></td>
				<td>- Template rendering functionality enables dynamic content generation by processing Jinja-style templates with provided parameters<br>- A core component for transforming static templates into customized output, it manages template parsing, parameter injection, and error handling<br>- Essential for generating dynamic files and content throughout the project's template-based operations.</td>
			</tr>
			</table>
		</blockquote>
	</details>
</details>

---
##  Getting Started

###  Prerequisites

Before getting started with tfjinja, ensure your runtime environment meets the following requirements:

- **Programming Language:** Rust
- **Package Manager:** Cargo


###  Installation

Install tfjinja using one of the following methods:

**Build from source:**

1. Clone the tfjinja repository:
```sh
❯ git clone ../tfjinja
```

2. Navigate to the project directory:
```sh
❯ cd tfjinja
```

3. Install the project dependencies:


**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```




###  Usage
Run tfjinja using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo run
```


###  Testing
Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```


---
##  Project Roadmap

- [X] **`Task 1`**: <strike>Implement feature one.</strike>
- [ ] **`Task 2`**: Implement feature two.
- [ ] **`Task 3`**: Implement feature three.

---

##  Contributing

- **💬 [Join the Discussions](https://LOCAL/20-RUST/tfjinja/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://LOCAL/20-RUST/tfjinja/issues)**: Submit bugs found or log feature requests for the `tfjinja` project.
- **💡 [Submit Pull Requests](https://LOCAL/20-RUST/tfjinja/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your LOCAL account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone C:\Private\20-RUST\tfjinja
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to LOCAL**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://LOCAL{/20-RUST/tfjinja/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=20-RUST/tfjinja">
   </a>
</p>
</details>

---

##  License

This project is protected under the [SELECT-A-LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

---
