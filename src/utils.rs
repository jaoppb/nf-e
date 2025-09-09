use quick_xml::{events::Event, Reader, Writer};
use std::{error::Error, io::Cursor};
use xml_canonicalization::Canonicalizer;

fn remove_whitespaces_from_xml(input: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = Reader::from_str(input);
    reader.config_mut().trim_text(true);

    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            Ok(Event::GeneralRef(e)) => writer.write_event(Event::GeneralRef(e))?,
            Ok(Event::Start(e)) => writer.write_event(Event::Start(e))?,
            Ok(Event::End(e)) => writer.write_event(Event::End(e))?,
            Ok(Event::Empty(e)) => writer.write_event(Event::Empty(e))?,
            Ok(Event::Text(e)) => writer.write_event(Event::Text(e))?,
            Ok(Event::Comment(e)) => writer.write_event(Event::Comment(e))?,
            Ok(Event::CData(e)) => writer.write_event(Event::CData(e))?,
            Ok(Event::Decl(e)) => writer.write_event(Event::Decl(e))?,
            Ok(Event::PI(e)) => writer.write_event(Event::PI(e))?,
            Ok(Event::DocType(e)) => writer.write_event(Event::DocType(e))?,

            Ok(Event::Eof) => break,

            Err(e) => return Err(Box::new(e)),
        }
    }

    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result).unwrap())
}

pub fn canonicalize_xml(input: &str) -> Result<String, Box<dyn Error>> {
    let mut result = Vec::new();
    let cleaned = remove_whitespaces_from_xml(input)?;
    Canonicalizer::read_from_str(&cleaned)
        .write_to_writer(Cursor::new(&mut result))
        .canonicalize(false)?;
    String::from_utf8(result).map_err(|e| e.into())
}

pub fn left_pad(input: &str, total_length: usize, pad_char: char) -> String {
    if input.len() >= total_length {
        input.to_string()
    } else {
        let padding = pad_char.to_string().repeat(total_length - input.len());
        format!("{}{}", padding, input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_canonicalize_str() {
        let input = r#"<root><child attribute="value">Text</child></root>"#;
        let expected = r#"<root><child attribute="value">Text</child></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }
}
