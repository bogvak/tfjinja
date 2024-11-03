use clap::Parser;
use config::{ConfigBuilder};
use std::fs::File;
use std::path::PathBuf;
use walkdir::WalkDir;

mod process_file; // Declares the module
mod render_jinja;
use process_file::process_file; // Imports the function

#[derive(Parser)]
struct Cli {
    /// The file extension to search for (e.g., .tf)
    #[arg(short, long, default_value = "")]
    extension: String,

    /// The prefix string to search for in the files
    #[arg(short, long, default_value = "")]
    prefix: String,
}

    fn load_parameter_from_settings(param: &str) -> Option<String> {
        let builder: ConfigBuilder<config::builder::DefaultState> = ConfigBuilder::default();
        let settings = builder
            .add_source(config::File::with_name("settings"))
            .build();
        if let Ok(config) = settings {
            // Read the key 'param' from the settings
            if let Ok(param_value) = config.get::<String>(param) {
                return Some(param_value);
            } else {
                eprintln!("Key 'param' not found or could not be read");
            };
        } else {
            eprintln!("Failed to load settings");
        }
        None
    }

fn find_files_with_extension(extension: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file()
            && entry.path().extension().and_then(|e| e.to_str()) == Some(extension)
        {
            results.push(entry.path().to_path_buf());
        }
    }
    results
}

fn process_files(file_path: &PathBuf, prefix: &str) {
    // Open the file
    if let Ok(file) = File::open(file_path) {
        process_file(file, file_path, prefix);
    } else {
        eprintln!("Could not open file: {}", file_path.display());
    }
}

fn main() {
    let args = Cli::parse();

    let mut extension = args.extension;
    let mut template_prefix = args.prefix;
    // Check if the extension is empty and try to read from settings
    if extension.is_empty() {
        extension =
            load_parameter_from_settings("file_extension").unwrap_or_else(|| "tf".to_string());
    }
    // Check if the prefix is empty and try to read from settings
    if template_prefix.is_empty() {
        template_prefix =
            load_parameter_from_settings("prefix").unwrap_or_else(|| "##tftpl:".to_string());
    }

    println!("Searching for files with extension: {}", extension);

    let files = find_files_with_extension(&extension);

    for file in files {
        println!("Processing file: {}", file.display());
        process_files(&file, &template_prefix);
    }
}
