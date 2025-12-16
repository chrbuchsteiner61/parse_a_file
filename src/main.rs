use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // List of elements to search for
    let elements = vec!["<a>", "<b>", "<c>","<d>","<e>"];
    
    // Read the file content
    let content = fs::read_to_string("C:\\Users\\chrbu\\Documents\\Development\\RUST\\ParseAFile\\parse_a_file\\src\\xml_style_file.xml")?;
    
    // New list to store found values
    let mut found_values = Vec::new();
    
    // Search for each element
    for element in &elements {
        // Create the closing tag (e.g., "<a>" -> "</a>")
        
        let closing_tag = element.replace("<", "</");
        println!("{}", closing_tag);
        // Find the value between opening and closing tags
        if let Some(value) = extract_value(&content, element, &closing_tag) {
            found_values.push(value);
            // println!("Found {} with value: {}", element, value);
        } else {
            println!("Element {} not found", element);
        }
    }
    
    println!("\nExtracted found values: {:?}", found_values);
    
    Ok(())
}

fn extract_value(content: &str, open_tag: &str, close_tag: &str) -> Option<String> {
    // Find the position of opening tag
    let start_pos = content.find(open_tag)?;
    let value_start = start_pos + open_tag.len();
    
    // Find the position of closing tag after the opening tag
    let remaining = &content[value_start..];
    let end_pos = remaining.find(close_tag)?;
    
    // Extract the value between tags
    let value = remaining[..end_pos].trim().to_string();
    Some(value)
}