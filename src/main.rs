use pdf_extract::extract_text;
use std::io;

/// Reads the PDF file and extracts text from it.
fn extract_pdf_text(file_path: &str) -> Result<String, io::Error> {
    match extract_text(file_path) {
        Ok(text) => Ok(text),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    }
}

/// Finds paragraphs containing the specified keywords (case insensitive).
/// 
/// # Arguments
/// * `text` - The text to search.
/// * `keywords` - A list of keywords to search for.
/// 
/// # Returns
/// A vector of paragraphs containing any of the keywords.
fn find_paragraphs_with_keywords(text: &str, keywords: &[&str]) -> Vec<String> {
    let mut matching_paragraphs = Vec::new();

    for paragraph in text.split("\n\n") {
        if keywords.iter().any(|&keyword| paragraph.to_lowercase().contains(&keyword.to_lowercase())) {
            matching_paragraphs.push(paragraph.to_string());
        }
    }

    matching_paragraphs
}

/// Main function to demonstrate usage.
fn main() {
    let pdf_path = "src/docs/sample.pdf"; // Replace with your PDF file path
    let keywords = [ "oxford"]; // Replace with your keywords

    match extract_pdf_text(pdf_path) {
        Ok(text) => {
            let matching_paragraphs = find_paragraphs_with_keywords(&text, &keywords);

            if matching_paragraphs.is_empty() {
                println!("No matching paragraphs found.");
            } else {
                println!("Paragraphs containing keywords: \n");
                for paragraph in matching_paragraphs {
                    println!("== paragraph start: \n{}\n === Paragraph end\n", paragraph);
                }
            }
        }
        Err(e) => {
            eprintln!("Error extracting text: {}", e);
        }
    }
}


/*Explaination */

/*

Here is a modular implementation in Rust that uses the `pdf-extract` crate to extract data from a PDF and identify the lines where specific keywords appear. The program is designed to be clean, reusable, and easy to extend.

### Explanation of the Code:

1. **`extract_pdf_text` Function**:
   - Reads a PDF file and extracts text using the `pdf-extract` crate.
   - Handles errors and returns a `Result<String, io::Error>`.

2. **`find_lines_with_keywords` Function**:
   - Splits the text into lines and searches for any of the specified keywords in each line.
   - Returns a `Vec<String>` containing all matching lines.

3. **Main Function**:
   - Demonstrates the usage of the above functions.
   - Prints the lines that contain any of the specified keywords or indicates if no matches were found.

### How to Use:
1. Install the `pdf-extract` crate in your project by adding it to your `Cargo.toml`:
   ```toml
   [dependencies]
   pdf-extract = "0.7"
   ```
2. Replace `"example.pdf"` with the path to your PDF file.
3. Replace `["keyword1", "keyword2", "keyword3"]` with your keywords.
4. Run the program to extract and find lines containing the keywords.

This modular approach ensures that you can reuse the functions for other projects or integrate them into larger applications.

 */