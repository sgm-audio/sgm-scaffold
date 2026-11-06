use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

/// Prompt for a string value with a default.
pub fn prompt_string(message: &str, default: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(default.to_string())
        .interact_text()
        .unwrap_or_else(|_| default.to_string())
}

/// Prompt for selection from a list.
pub fn prompt_select(message: &str, items: &[&str], default: usize) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .items(items)
        .default(default)
        .interact()
        .unwrap_or(default)
}

/// Prompt for yes/no confirmation.
pub fn prompt_confirm(message: &str, default: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .default(default)
        .interact()
        .unwrap_or(default)
}

/// Print a success message.
pub fn print_success(msg: &str) {
    println!("{}", style(msg).green().bold());
}

/// Print an info message.
pub fn print_info(msg: &str) {
    println!("{}", style(msg).cyan());
}

/// Print a warning message.
pub fn print_warning(msg: &str) {
    println!("{}", style(msg).yellow());
}

/// Print an error message.
pub fn print_error(msg: &str) {
    eprintln!("{}", style(msg).red().bold());
}