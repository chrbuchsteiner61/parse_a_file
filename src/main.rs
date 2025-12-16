use std::fs;
use std::io;

const FILE_PATH: &str = "src/xml_style_file.xml";

fn main() -> io::Result<()> {
    let elements = vec!["<a>", "<b>", "<c>", "<d>", "<e>", "<f>", "<g>"];
    let content = fs::read_to_string(FILE_PATH)?;
    
    let values = process_elements(&content, &elements);
    
    println!("\nExtracted values: {:?}", values);
    
    Ok(())
}

fn process_elements(content: &str, elements: &[&str]) -> Vec<String> {
    let mut values = Vec::new();
    
    for element in elements {
        let element_name = extract_element_name(element);
        let closing_tag = create_closing_tag(element);
        let found_values = extract_all_values(content, element, &closing_tag);
        
        match process_element_result(content, element, &element_name, found_values) {
            ElementResult::Values(vals) => {
                for value in vals {
                    let formatted = format!("{}: {}", element_name, value);
                    println!("Found {}", formatted);
                    values.push(formatted);
                }
            }
            ElementResult::Headline(headline) => {
                println!("Found headline: {}", headline);
                values.push(headline);
            }
            ElementResult::NotFound => {
                println!("Element '{}' not found in file", element_name);
            }
        }
    }
    
    values
}

enum ElementResult {
    Values(Vec<String>),
    Headline(String),
    NotFound,
}

fn process_element_result(
    content: &str,
    element: &str,
    element_name: &str,
    found_values: Vec<String>,
) -> ElementResult {
    if found_values.is_empty() {
        if content.contains(element) {
            ElementResult::Headline(element_name.to_string())
        } else {
            ElementResult::NotFound
        }
    } else {
        ElementResult::Values(found_values)
    }
}

fn extract_element_name(element: &str) -> String {
    element.trim_start_matches('<').trim_end_matches('>').to_string()
}

fn create_closing_tag(open_tag: &str) -> String {
    open_tag.replace('<', "</")
}

fn extract_all_values(content: &str, open_tag: &str, close_tag: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut search_from = 0;
    
    while let Some(tag_position) = find_next_tag(content, open_tag, search_from) {
        let value_start = tag_position + open_tag.len();
        
        if let Some(end_pos) = content[value_start..].find(close_tag) {
            let value_content = &content[value_start..value_start + end_pos];
            
            if let Some(extracted) = extract_text_content(value_content) {
                results.push(extracted);
            }
            
            search_from = value_start + end_pos + close_tag.len();
        } else {
            break;
        }
    }
    
    results
}

fn find_next_tag(content: &str, tag: &str, from: usize) -> Option<usize> {
    content[from..].find(tag).map(|pos| from + pos)
}

fn extract_text_content(value_content: &str) -> Option<String> {
    if value_content.contains('<') {
        // Has nested content, extract only direct text before first tag
        value_content
            .find('<')
            .and_then(|first_tag| {
                let direct_text = value_content[..first_tag].trim();
                if !direct_text.is_empty() {
                    Some(direct_text.to_string())
                } else {
                    None
                }
            })
    } else {
        // Simple text content, no nesting
        let text = value_content.trim();
        if !text.is_empty() {
            Some(text.to_string())
        } else {
            None
        }
    }
}