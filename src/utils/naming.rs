use crate::error::{Result, ScaffoldError};

/// Validate a project name (must be kebab-case, valid Rust identifier).
pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(ScaffoldError::InvalidName("Name cannot be empty".into()));
    }
    if !name
        .chars()
        .all(|c| c.is_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(ScaffoldError::InvalidName(
            "Name must be kebab-case (lowercase, digits, hyphens only)".into(),
        ));
    }
    if name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return Err(ScaffoldError::InvalidName(
            "Invalid hyphen placement".into(),
        ));
    }
    let snake = to_snake_case(name);
    if snake.is_empty() || snake.starts_with(|c: char| c.is_ascii_digit()) {
        return Err(ScaffoldError::InvalidName(
            "Name must start with a letter".into(),
        ));
    }
    let reserved = [
        "self", "super", "crate", "mod", "fn", "let", "mut", "pub", "use", "trait", "impl",
        "struct", "enum", "type", "where", "async", "await", "dyn", "move", "ref", "static",
        "const", "extern", "unsafe", "match", "if", "else", "for", "while", "loop", "return",
        "break", "continue", "in",
    ];
    if reserved.contains(&snake.as_str()) {
        return Err(ScaffoldError::InvalidName(format!(
            "'{}' is a reserved word",
            snake
        )));
    }
    Ok(())
}

/// Convert kebab-case to snake_case.
pub fn to_snake_case(name: &str) -> String {
    name.replace('-', "_")
}

/// Convert kebab-case to PascalCase.
pub fn to_pascal_case(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Convert kebab-case to a display name (e.g., "sgm-drift" → "SGM Drift").
pub fn to_display_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a unique 16-byte CLAP/VST3 class ID as a Rust byte array literal.
pub fn generate_class_id() -> String {
    let bytes: [u8; 16] = rand::random();
    // Format: *b"SgmXXXXXXXXXXXX" (first 3 bytes are "Sgm", rest random hex-ish)
    let suffix: String = bytes[3..]
        .iter()
        .map(|b| {
            let c = *b % 36;
            if c < 10 {
                (b'0' + c) as char
            } else {
                (b'A' + c - 10) as char
            }
        })
        .collect();
    format!("*b\"Sgm{}\"", &suffix[..13])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_name() {
        assert!(validate_name("drift").is_ok());
        assert!(validate_name("sgm-prism").is_ok());
        assert!(validate_name("my-plugin-2").is_ok());
    }

    #[test]
    fn test_validate_invalid_name() {
        assert!(validate_name("").is_err());
        assert!(validate_name("Drift").is_err());
        assert!(validate_name("my_plugin").is_err());
        assert!(validate_name("-drift").is_err());
        assert!(validate_name("drift-").is_err());
        assert!(validate_name("drift--eq").is_err());
        assert!(validate_name("fn").is_err());
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("sgm-drift"), "sgm_drift");
        assert_eq!(to_snake_case("drift"), "drift");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("sgm-drift"), "SgmDrift");
        assert_eq!(to_pascal_case("drift"), "Drift");
    }

    #[test]
    fn test_to_display_name() {
        assert_eq!(to_display_name("sgm-drift"), "Sgm Drift");
        assert_eq!(to_display_name("drift"), "Drift");
    }

    #[test]
    fn test_generate_class_id() {
        let id = generate_class_id();
        assert!(id.starts_with("*b\"Sgm"));
        assert!(id.ends_with("\""));
    }
}