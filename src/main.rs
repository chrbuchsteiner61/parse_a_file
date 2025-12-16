use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // List of elements to search for
    let elements = vec!["<a>", "<b>", "<c>", "<d>", "<e>", "<f>"];
    
    // Read the file content
    let content = fs::read_to_string("src/xml_style_file.xml")?;
    
       // New list to store found values
    let mut values = Vec::new();
    
    // Search for each element
    for element in &elements {
        // Create the closing tag (e.g., "<a>" -> "</a>")
        let closing_tag = element.replace("<", "</");
        
        // Extract element name (e.g., "<a>" -> "a")
        let element_name = element.trim_start_matches('<').trim_end_matches('>');
        
        // Find all values for this element
        let found_values = extract_all_values(&content, element, &closing_tag);
        
        if found_values.is_empty() {
            // No direct value - check if element exists and use as headline
            if content.contains(element) {
                let headline = element_name.to_string();
                values.push(headline.clone());
                println!("Found headline: {}", headline);
            }
        } else {
            for value in found_values {
                let formatted = format!("{}: {}", element_name, value);
                values.push(formatted.clone());
                println!("Found {}", formatted);
            }
        }
    }
    
    println!("\nExtracted values: {:?}", values);
    
    Ok(())
}

fn extract_all_values(content: &str, open_tag: &str, close_tag: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut search_from = 0;
    
    while let Some(start_pos) = content[search_from..].find(open_tag) {
        let abs_start = search_from + start_pos;
        let value_start = abs_start + open_tag.len();
        
        // Find the closing tag
        if let Some(end_pos) = content[value_start..].find(close_tag) {
            let value_content = &content[value_start..value_start + end_pos];
            
            // Check if there are nested tags
            if value_content.contains('<') {
                // Has nested content, extract only direct text (before first tag)
                if let Some(first_tag) = value_content.find('<') {
                    let direct_text = value_content[..first_tag].trim();
                    if !direct_text.is_empty() {
                        results.push(direct_text.to_string());
                    }
                }
                // If no direct text before nested tags, skip this element
            } else {
                // Simple text content, no nesting
                let text = value_content.trim();
                if !text.is_empty() {
                    results.push(text.to_string());
                }
            }
            
            search_from = value_start + end_pos + close_tag.len();
        } else {
            break;
        }
    }
    
    results
}