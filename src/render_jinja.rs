use tera::{Tera, Context};
use std::{collections::HashMap};

pub fn render_template(template_content: &str, template_name: &str, params: HashMap<String, String>) -> Option<String> {
    let mut tera = Tera::default();

    // Add the template content directly
    if tera.add_raw_template(template_name, template_content).is_err() {
        eprintln!("Failed to add template");
        return None;
    }

    let context_data = create_context(params);

    // Render the template with the given context
    match tera.render(template_name, &context_data) {
        Ok(rendered) => Some(rendered),
        Err(err) => {
            eprintln!("Failed to render template: {}", err);
            None
        }
    }
}

fn create_context(params: HashMap<String, String>) -> Context {
    let mut context = Context::new();
    for (key, value) in params {
        context.insert(&key, &value); // Insert each parameter into the context
    }
    context // Return the populated context
}