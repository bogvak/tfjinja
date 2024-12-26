use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

// mod crate::render_jinja;
use crate::render_jinja::render_template;

pub fn process_file(file: File, file_path: &PathBuf, prefix: &str) {
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut need_to_include: bool = true;

    for line in reader.lines() {
        if let Ok(content) = line {
            // Check for the prefix, ignoring leading spaces
            let trimmed_content = content.trim_start();
            if trimmed_content.starts_with(prefix) {
                let stripped = trimmed_content
                    .strip_prefix(prefix)
                    .map(|token| token.trim())
                    .unwrap_or("");
                match stripped {
                    ">>" => {
                        need_to_include = false;
                        continue;
                    }
                    "<<" => {
                        need_to_include = true;
                        continue;
                    }
                    _ => {} // Do nothing for other cases
                }
                lines.push(content.clone());
                add_template_to_file(
                    content.clone(),
                    trimmed_content,
                    file_path,
                    prefix,
                    &mut lines,
                );
                continue;
            }

            // Store the current line in the output
            if need_to_include {
                lines.push(content.clone())
            };
        }
    }

    // Write the modified content back to the original file
    if let Ok(mut output_file) = File::create(file_path) {
        for line in lines {
            writeln!(output_file, "{}", line).expect("Failed to write to file");
        }
    } else {
        eprintln!("Could not open file for writing: {}", file_path.display());
    }
}

fn read_external_file_with_indentation(
    file_path: &PathBuf,
    indentation: &str,
    tpl_params: HashMap<String, String>,
) -> Result<Vec<String>, io::Error> {
    let mut content_lines = Vec::new();

    if let Ok(external_file) = File::open(file_path) {
        let external_reader = BufReader::new(external_file);
        for line in external_reader.lines() {
            if let Ok(external_content) = line {
                // Apply the indentation to each line and store it
                content_lines.push(format!("{}{}", indentation, external_content));
            }
        }
    } else {
        eprintln!("Could not open external file: {}", file_path.display());
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to open external file",
        ));
    }

    // let prm: HashMap<String, String> = HashMap::from([
    //     // ("cpu_limit".to_string(), "200m".to_string()),
    // ]);

    let template_content = content_lines.join("\n");
    let rendered_content =
        render_template(&template_content, "test", tpl_params).unwrap_or(template_content);

    Ok(vec![rendered_content])
}

fn read_external_external_template_with_param(
    command: &str,
    indentation: &str,
    current_file_path: &PathBuf,
) -> Result<Vec<String>, io::Error> {
    let parameters = parse_template_string(command);
    // Print the resulting HashMap
    for (key, value) in &parameters {
        println!("{}: {}", key, value);
    }

    let external_file_path = &parameters["__path"].trim_start();
    let path = current_file_path.parent().unwrap().join(external_file_path);
    match fs::canonicalize(&path) {
        Ok(resolved_path) => {
            println!("Resolved path: {:?}", resolved_path);
            read_external_file_with_indentation(&resolved_path, indentation, parameters)
        }
        Err(e) => {
            println!("{}", path.display());
            eprintln!("Error resolving path: {}", e);
            Err(e)
        }
    }
}

fn parse_template_string(input: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    // Split the input by semicolon
    let parts: Vec<&str> = input.split(';').map(|s| s.trim()).collect();

    // The first part is the template path
    if let Some(path) = parts.get(0) {
        result.insert("__path".to_string(), path.to_string());
    }

    // Process the rest of the parts to get parameters
    for part in &parts[1..] {
        if let Some((key, value)) = part.split_once('=') {
            result.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    result
}

fn add_template_to_file(
    content: String,
    trimmed_content: &str,
    file_path: &PathBuf,
    prefix: &str,
    lines: &mut Vec<String>,
) {
    // Capture the leading whitespace for indentation
    let leading_whitespace = &content[..content.find(trimmed_content).unwrap_or(0)];

    // Extract the string after the prefix
    if let Some(command) = trimmed_content.strip_prefix(prefix) {
        // Retrieve content with indentation
        match read_external_external_template_with_param(&command, leading_whitespace, file_path) {
            Ok(external_lines) => {
                lines.push(format!(
                    "{}{}{}",
                    leading_whitespace,
                    prefix,
                    String::from(">>")
                ));
                lines.extend(external_lines);
                lines.push(format!(
                    "{}{}{}",
                    leading_whitespace,
                    prefix,
                    String::from("<<")
                ));
            }
            Err(_) => eprintln!("Could not read content from {}", command),
        }
    }
}
