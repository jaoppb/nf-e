use std::io::Cursor;
use xml_canonicalization::Canonicalizer;

pub fn canonicalize_str(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    Canonicalizer::read_from_str(input)
        .write_to_writer(Cursor::new(&mut result))
        .canonicalize(false)?;
    String::from_utf8(result).map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_canonicalize_str() {
        let input = r#"<root><child attribute="value">Text</child></root>"#;
        let expected = r#"<root><child attribute="value">Text</child></root>"#;
        
        match canonicalize_str(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }
}