use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Value type for frontmatter updates
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FrontmatterValue {
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<String>),
    Null,
}

impl FrontmatterValue {
    pub fn to_yaml_value(&self) -> String {
        match self {
            FrontmatterValue::String(s) => {
                if s.contains(':') || s.contains('#') || s.contains('\n') || 
                   s.starts_with('[') || s.starts_with('{') ||
                   s == "true" || s == "false" || s == "null" ||
                   s.parse::<f64>().is_ok() {
                    format!("\"{}\"", s.replace('\"', "\\\""))
                } else {
                    s.clone()
                }
            }
            FrontmatterValue::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            FrontmatterValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            FrontmatterValue::List(items) => {
                if items.is_empty() {
                    "[]".to_string()
                } else {
                    items.iter()
                        .map(|item| {
                            let quoted = if item.contains(':') || item.starts_with('[') || item.starts_with('{') {
                                format!("\"{}\"", item.replace('\"', "\\\""))
                            } else {
                                format!("\"{}\"", item)
                            };
                            format!("  - {}", quoted)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                }
            }
            FrontmatterValue::Null => "null".to_string(),
        }
    }
}

/// Format a key for YAML output (quote if necessary)
pub fn format_yaml_key(key: &str) -> String {
    if key.contains(' ') || key.contains(':') || key.contains('#') || 
       key.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_' && c != '-') {
        format!("\"{}\"", key)
    } else {
        key.to_string()
    }
}

/// Check if a line defines a specific key (handles quoted and unquoted keys)
fn line_is_key(line: &str, key: &str) -> bool {
    let trimmed = line.trim_start();
    
    if trimmed.starts_with(key) && trimmed[key.len()..].starts_with(':') {
        return true;
    }
    
    let dq = format!("\"{}\":", key);
    if trimmed.starts_with(&dq) {
        return true;
    }
    
    let sq = format!("'{}\':", key);
    if trimmed.starts_with(&sq) {
        return true;
    }
    
    false
}

/// Internal function to update frontmatter content
pub fn update_frontmatter_content(content: &str, key: &str, value: Option<FrontmatterValue>) -> Result<String, String> {
    if !content.starts_with("---\n") {
        return match value {
            Some(v) => {
                let yaml_key = format_yaml_key(key);
                let yaml_value = v.to_yaml_value();
                let fm = if yaml_value.contains('\n') {
                    format!("---\n{}:\n{}\n---\n", yaml_key, yaml_value)
                } else {
                    format!("---\n{}: {}\n---\n", yaml_key, yaml_value)
                };
                Ok(format!("{}{}", fm, content))
            }
            None => Ok(content.to_string()),
        };
    }
    
    let fm_end = content[4..].find("\n---")
        .map(|i| i + 4)
        .ok_or_else(|| "Malformed frontmatter: no closing ---".to_string())?;
    
    let fm_content = &content[4..fm_end];
    let rest = &content[fm_end + 4..];
    
    let lines: Vec<&str> = fm_content.lines().collect();
    let mut new_lines: Vec<String> = Vec::new();
    let mut found_key = false;
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        if line_is_key(line, key) {
            found_key = true;
            i += 1;
            while i < lines.len() && (lines[i].starts_with("  - ") || lines[i].trim().is_empty()) {
                if lines[i].trim().is_empty() {
                    break;
                }
                i += 1;
            }
            
            if let Some(ref v) = value {
                let yaml_key = format_yaml_key(key);
                let yaml_value = v.to_yaml_value();
                if yaml_value.contains('\n') {
                    new_lines.push(format!("{}:", yaml_key));
                    new_lines.push(yaml_value);
                } else {
                    new_lines.push(format!("{}: {}", yaml_key, yaml_value));
                }
            }
            continue;
        }
        
        new_lines.push(line.to_string());
        i += 1;
    }
    
    if !found_key {
        if let Some(ref v) = value {
            let yaml_key = format_yaml_key(key);
            let yaml_value = v.to_yaml_value();
            if yaml_value.contains('\n') {
                new_lines.push(format!("{}:", yaml_key));
                new_lines.push(yaml_value);
            } else {
                new_lines.push(format!("{}: {}", yaml_key, yaml_value));
            }
        }
    }
    
    let new_fm = new_lines.join("\n");
    Ok(format!("---\n{}\n---{}", new_fm, rest))
}

/// Helper to read a file, apply a frontmatter transformation, and write back.
pub fn with_frontmatter<F>(path: &str, transform: F) -> Result<String, String>
where
    F: FnOnce(&str) -> Result<String, String>,
{
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    
    let updated = transform(&content)?;
    
    fs::write(file_path, &updated)
        .map_err(|e| format!("Failed to write {}: {}", path, e))?;
    
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_frontmatter_string() {
        let content = "---\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "Status", Some(FrontmatterValue::String("Active".to_string()))).unwrap();
        assert!(updated.contains("Status: Active"));
        assert!(!updated.contains("Status: Draft"));
    }

    #[test]
    fn test_update_frontmatter_add_new_key() {
        let content = "---\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "Owner", Some(FrontmatterValue::String("Luca".to_string()))).unwrap();
        assert!(updated.contains("Owner: Luca"));
        assert!(updated.contains("Status: Draft"));
    }

    #[test]
    fn test_update_frontmatter_quoted_key() {
        let content = "---\n\"Is A\": Note\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "Is A", Some(FrontmatterValue::String("Project".to_string()))).unwrap();
        assert!(updated.contains("\"Is A\": Project"));
        assert!(!updated.contains("Note"));
    }

    #[test]
    fn test_update_frontmatter_list() {
        let content = "---\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "aliases", Some(FrontmatterValue::List(vec!["Alias1".to_string(), "Alias2".to_string()]))).unwrap();
        assert!(updated.contains("aliases:"));
        assert!(updated.contains("  - \"Alias1\""));
        assert!(updated.contains("  - \"Alias2\""));
    }

    #[test]
    fn test_update_frontmatter_replace_list() {
        let content = "---\naliases:\n  - Old1\n  - Old2\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "aliases", Some(FrontmatterValue::List(vec!["New1".to_string()]))).unwrap();
        assert!(updated.contains("  - \"New1\""));
        assert!(!updated.contains("Old1"));
        assert!(!updated.contains("Old2"));
        assert!(updated.contains("Status: Draft"));
    }

    #[test]
    fn test_delete_frontmatter_property() {
        let content = "---\nStatus: Draft\nOwner: Luca\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "Owner", None).unwrap();
        assert!(!updated.contains("Owner"));
        assert!(updated.contains("Status: Draft"));
    }

    #[test]
    fn test_delete_frontmatter_list_property() {
        let content = "---\naliases:\n  - Alias1\n  - Alias2\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "aliases", None).unwrap();
        assert!(!updated.contains("aliases"));
        assert!(!updated.contains("Alias1"));
        assert!(updated.contains("Status: Draft"));
    }

    #[test]
    fn test_update_frontmatter_no_existing() {
        let content = "# Test\n\nSome content here.";
        let updated = update_frontmatter_content(content, "Status", Some(FrontmatterValue::String("Draft".to_string()))).unwrap();
        assert!(updated.starts_with("---\n"));
        assert!(updated.contains("Status: Draft"));
        assert!(updated.contains("# Test"));
    }

    #[test]
    fn test_update_frontmatter_bool() {
        let content = "---\nStatus: Draft\n---\n# Test\n";
        let updated = update_frontmatter_content(content, "Reviewed", Some(FrontmatterValue::Bool(true))).unwrap();
        assert!(updated.contains("Reviewed: true"));
    }

    #[test]
    fn test_format_yaml_key_simple() {
        assert_eq!(format_yaml_key("Status"), "Status");
        assert_eq!(format_yaml_key("is_a"), "is_a");
    }

    #[test]
    fn test_format_yaml_key_with_spaces() {
        assert_eq!(format_yaml_key("Is A"), "\"Is A\"");
        assert_eq!(format_yaml_key("Created at"), "\"Created at\"");
    }
}
