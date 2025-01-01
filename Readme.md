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
- [Usage](#usage)
  - [Overview](#overview)
  - [Example Use Case](#example-use-case)
    - [Template Definition](#template-definition)
    - [Rendered Content](#rendered-content)
    - [Key Features](#key-features)
    - [How It Works](#how-it-works)
    - [Example Run](#example-run)
  - [Installation](#installation)
    - [1. Download Precompiled Binaries](#1-download-precompiled-binaries)
    - [2. Build from Source](#2-build-from-source)
      - [Prerequisites](#prerequisites)
      - [Steps to Build](#steps-to-build)
  - [Notes on templating format](#notes-on-templating-format)
      - [Template Engine: Tera](#template-engine-tera)
      - [Supported Template Syntax](#supported-template-syntax)
      - [Template Compatibility](#template-compatibility)
- [Features](#features)
- [Project Structure](#project-structure)
  - [Project Index](#project-index)
- [Extending and contributing](#extending-and-contributing)
  - [Testing](#testing)
- [Project Roadmap](#project-roadmap)
- [Contributing](#contributing)
- [License](#license)
  - [Highlights of the MIT License:](#highlights-of-the-mit-license)

---

tfjinja is a specialized command-line tool that streamlines Terraform infrastructure management by enabling Jinja-style templating in Terraform files. It helps DevOps teams reduce code duplication and maintain consistency by allowing dynamic content generation, parameter substitution, and template reuse across infrastructure configurations. Perfect for organizations managing complex, multi-environment Terraform deployments.

---

## Usage

### Overview
This utility helps with templating in Terraform projects where conditional inclusion of nested blocks is not straightforward. For example, if you have a Kubernetes setup described in Terraform with multiple deployments and need to add a sidecar container (e.g., [Socat](https://www.kali.org/tools/socat/) container) to each deployment, this tool simplifies the process.

The utility processes templates and includes their rendered content into Terraform files. It uses a specific template definition string (`##tftpl` by default) to identify templates and their associated parameters.

Below is a step-by-step example of how to use this tool.

---

### Example Use Case

#### Template Definition
Suppose you have a sidecar container template defined in the file [examples/socat_container.tftpl](examples/socat_container.tftpl). You can include it in your Terraform deployment configuration using the following syntax:

```hcl
##tftpl: ../_shared/socat_container.tftpl; add_sec_context=true; sec_id=401;
```
This defines a template inclusion string. Parameters like `add_sec_context` and `sec_id` can be passed to the template for rendering.

#### Rendered Content
When the utility runs, it renders the content of the template and includes it between two Terraform comments. For example:

```hcl
##tftpl:>>
# Sidecar socat container
# command = ["socat", "TCP-LISTEN:10000,fork,reuseaddr", "TCP:mysql.mysql.svc.cluster.local:3306"]
container {
    name  = "${local.app_name}-socat-container"
    image = "alpine/socat"

    volume_mount {
    name       = "mysql-socket"
    mount_path = "/var/run/mysqld/"
    }

    command = ["socat", "UNIX-LISTEN:/var/run/mysqld/mysqld.sock,fork,reuseaddr", "TCP4:mysql.mysql.svc.cluster.local:3306"]
    resources {
    limits = {
        cpu    = "100m"
        memory = "128Mi"
    }
    requests = {
        cpu    = "100m"
        memory = "64Mi"
    }
    }

    security_context {
    run_as_group = "401"
    run_as_user  = "401"
    }
}
##tftpl:<<
```
The content is injected in such a way that the Terraform files remain valid and usable as normal Terraform configurations.

#### Key Features

- **Comment-Based Templating:** The `##tftpl` comments ensure that the tool does not interfere with Terraform syntax or functionality. All files remain valid `.tf` files.
- **Automatic Updates:** Whenever the utility is re-run, it re-renders templates and updates the content in `.tf` files. This ensures that changes to templates propagate to all affected Terraform configurations.
- **Parameterization:** You can pass parameters to templates via the definition string (e.g., `add_sec_context=true; sec_id=401`)

#### How It Works

1. **Scan and Process Files**
The utility scans all Terraform files (`*.tf` by default) in the current directory and all subdirectories. You can customize the file extension by passing the `--extension` argument or updating the `settings.tom`l configuration file.

2. **Template Path Resolution**
The tool locates templates based on paths **relative to the processed** `.tf` **file**. For example, the template path `../_shared/socat_container.tftpl` is resolved relative to the Terraform file in which the `##tftpl` string is defined.

3. **Settings Configuration**
-- Use `settings.toml` to define global configuration options, such as the template prefix (`prefix`) or file extension to scan (`extension`).
-- Override settings using command-line arguments.
4. **Rendering and Inclusion**
The utility renders the content of the template with the provided parameters and includes the rendered output between the `##tftpl:>>` and `##tftpl:<<` comments in the appropriate `.tf` file.

#### Example Run
To process all .tf files in the current directory with the default configuration:

```bash
tfjinja
```
To process files with a custom extension or prefix:
```bash
tfjinja --extension .terraform --prefix ##customtpl:
```
For detailed configuration, refer to the settings.toml file.

### Installation

#### 1. Download Precompiled Binaries
You can download the latest precompiled binaries for your platform from the [Releases page](https://github.com/bogvak/tfjinja/releases) on GitHub. Precompiled binaries are available for common platforms like Windows, macOS, and Linux.

Once downloaded, place the binary in a directory included in your `PATH` to run it from the command line.

#### 2. Build from Source
If you'd prefer to build the utility from source, follow these steps:

##### Prerequisites
- [Rust](https://www.rust-lang.org/) installed (with Cargo, the Rust package manager).
- A supported C compiler for your platform.

##### Steps to Build
1. Clone the repository:
```bash
   git clone https://github.com/bogvak/tfjinja.git
   cd tfjinja
```
2. Build the binary:

```bash
cargo build --release
```
3. The binary will be available in the target/release/ directory:
```bash
./target/release/
```
4. Add the binary to your PATH to use it globally.

### Notes on templating format

##### Template Engine: Tera

Tfjinja is powered by the [Tera](https://github.com/Keats/tera) Rust library, a fast and powerful template engine inspired by Jinja2, making it flexible and easy to use for template-based operations.

##### Supported Template Syntax
Since Tera is inspired by Jinja2, it supports the familiar syntax and features such as:
- Variable interpolation: `{{ variable_name }}`
- Conditionals: `{% if condition %} ... {% endif %}`
- Loops: `{% for item in collection %} ... {% endfor %}`
- Filters and functions for data transformation
- Includes and inheritance for reusable templates

##### Template Compatibility
- Tfjinja templates are written in Tera format.
- Ensure that your templates adhere to Tera's supported syntax and features to avoid rendering issues.
- If you're migrating from Jinja2, most templates can be reused with minimal changes.

You can learn more about Tera syntax and capabilities in the [Tera documentation](https://github.com/Keats/tera).

---

##  Features

|      | Feature         | Summary       |
| :--- | :---:           | :---          |
| ⚙️  | **Architecture**  | <ul><li>Built in `Rust` with a modular design focusing on template processing</li><li>Core components split into `main.rs`, `process_file.rs`, and `render_jinja.rs`</li><li>Follows Terraform-compatible template processing architecture</li></ul> |
| 🔩 | **Code Quality**  | <ul><li>Implements Rust 2021 edition standards</li><li>Clear separation of concerns between file processing and template rendering</li><li>Structured error handling throughout the codebase</li></ul> |
| 📄 | **Configuration** | <ul><li>Configuration of app in `settings.toml`</li><li>Example templates provided in `examples/` directory</li><li>Clear package manifest documentation in `Cargo.toml`</li></ul> |
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
				<td><b><a href='https://github.com/bogvak/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
				<td>- Project manifest configuration defining core dependencies and metadata for tfjinja, a Rust-based application<br>- Specifies essential crates including clap for command-line argument parsing, walkdir for filesystem traversal, config for configuration management, and tera for template processing<br>- Sets project version to 0.1.0 and targets Rust 2021 edition standards.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/bogvak/blob/master/settings.toml'>settings.toml</a></b></td>
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
				<td><b><a href='https://github.com/bogvak/blob/master/examples/socat_container.tftpl'>socat_container.tftpl</a></b></td>
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
				<td><b><a href='https://github.com/bogvak/blob/master/src/main.rs'>main.rs</a></b></td>
				<td>- Serves as the entry point for a template processing utility that recursively scans directories for files with specified extensions<br>- Manages command-line arguments and configuration settings to identify template files, processes them using Jinja-style templating, and handles file operations<br>- Coordinates between different modules to enable template rendering and file processing functionality.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/bogvak/blob/master/src/process_file.rs'>process_file.rs</a></b></td>
				<td>- Processes and manages file content transformations by handling template inclusions and parameter substitutions<br>- Supports dynamic file content injection with customizable indentation while maintaining the original file structure<br>- Enables template rendering with variable parameters and provides mechanisms for external file inclusion, making it a core component for file-based template processing operations.</td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/bogvak/blob/master/src/render_jinja.rs'>render_jinja.rs</a></b></td>
				<td>- Template rendering functionality enables dynamic content generation by processing Jinja-style templates with provided parameters<br>- A core component for transforming static templates into customized output, it manages template parsing, parameter injection, and error handling<br>- Essential for generating dynamic files and content throughout the project's template-based operations.</td>
			</tr>
			</table>
		</blockquote>
	</details>
</details>

---
##  Extending and contributing
###  Testing
Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```


---
##  Project Roadmap

- [ ] Add unit tests.

---

##  Contributing

<!-- - **💬 [Join the Discussions](https://LOCAL/20-RUST/tfjinja/discussions)**: Share your insights, provide feedback, or ask questions. -->
- **🐛 [Report Issues]((https://github.com/bogvak/tfjinja/issues))**: Submit bugs found or log feature requests for the `tfjinja` project.
<!-- - **💡 [Submit Pull Requests](https://LOCAL/20-RUST/tfjinja/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs. -->

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your LOCAL account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/bogvak/tfjinja.git
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

<!-- <details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://LOCAL{/20-RUST/tfjinja/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=20-RUST/tfjinja">
   </a>
</p>
</details> -->

---

## License

This project is licensed under the MIT License.

You are free to use, modify, and distribute this software in any way, as long as the original license and copyright notice are included in all copies or substantial portions of the software.

For more details, see the [LICENSE](LICENSE) file included in this repository.

### Highlights of the MIT License:
- **Permission**: Allows private use, modification, distribution, and sublicensing.
- **Attribution**: Requires that the original license and copyright notice are included.
- **Liability**: Comes with no warranties, and the authors are not liable for any damages.

This permissive license ensures Tfjinja can be widely adopted while remaining developer-friendly.


---

<!-- ##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

--- -->
