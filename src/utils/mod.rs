mod canonicalizer;

use canonicalizer::Canonicalizer;

use std::io::Cursor;

pub fn canonicalize_xml(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut reader = quick_xml::Reader::from_str(input);
    let config = reader.config_mut();
    config.trim_text(true);
    config.expand_empty_elements = true;

    let mut result = Vec::new();
    let mut canonicalizer = Canonicalizer::new(Cursor::new(&mut result));
    canonicalizer.canonicalize_events(&mut reader)?;

    Ok(String::from_utf8_lossy(&result).to_string())
}

pub fn left_pad(input: &str, total_length: usize, pad_char: char) -> String {
    if input.len() >= total_length {
        input.to_string()
    } else {
        let padding = pad_char.to_string().repeat(total_length - input.len());
        format!("{}{}", padding, input)
    }
}

pub fn hash_sha1(input: &str) -> Vec<u8> {
    use sha1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

pub fn sign_sha1(
    input: &str,
    private_key: &rsa::RsaPrivateKey,
) -> Result<Vec<u8>, rsa::errors::Error> {
    use rsa::{pkcs1v15::Pkcs1v15Sign, traits::SignatureScheme};
    use sha1::Sha1;

    let signer = Pkcs1v15Sign::new::<Sha1>();
    signer.sign::<rsa::rand_core::OsRng>(None, &private_key, &hash_sha1(input))
}

pub fn verify_sha1(
    input: &str,
    signature: &[u8],
    public_key: &rsa::RsaPublicKey,
) -> Result<(), rsa::errors::Error> {
    use rsa::{pkcs1v15::Pkcs1v15Sign, traits::SignatureScheme};
    use sha1::Sha1;

    let verifier = Pkcs1v15Sign::new::<Sha1>();
    verifier.verify(&public_key, &hash_sha1(input), signature)
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

    #[test]
    fn test_canonicalize_empty_tag() {
        let input = r#"<root><empty /></root>"#;
        let expected = r#"<root><empty></empty></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    #[test]
    fn test_canonicalize_attributes_order() {
        let input = r#"<root><child b="2" a="1">Text</child></root>"#;
        let expected = r#"<root><child a="1" b="2">Text</child></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    #[test]
    fn test_canonicalize_empty_tag_with_attributes() {
        let input = r#"<root><empty attr="value"/></root>"#;
        let expected = r#"<root><empty attr="value"></empty></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    #[test]
    fn test_canonicalize_cdata() {
        let input = r#"<root><data><![CDATA[Some <cdata> content]]></data></root>"#;
        let expected = r#"<root><data>Some &lt;cdata&gt; content</data></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    #[test]
    fn test_canonicalize_namespace() {
        let input = r#"<root xmlns:ns="http://example.com/ns"><ns:child>Text</ns:child></root>"#;
        let expected = r#"<root xmlns:ns="http://example.com/ns"><ns:child>Text</ns:child></root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    fn test_canonicalize_mixed_content() {
        let input = r#"<root>Some <b>bold</b> and <i>italic</i> text.</root>"#;
        let expected = r#"<root>Some <b>bold</b> and <i>italic</i> text.</root>"#;

        match canonicalize_xml(input) {
            Ok(output) => assert_eq!(output, expected),
            Err(e) => panic!("Error during canonicalization: {}", e),
        }
    }

    #[test]
    fn test_left_pad() {
        let input = "123";
        let padded = left_pad(input, 5, '0');
        assert_eq!(padded, "00123");

        let input2 = "12345";
        let padded2 = left_pad(input2, 5, '0');
        assert_eq!(padded2, "12345");

        let input3 = "123456";
        let padded3 = left_pad(input3, 5, '0');
        assert_eq!(padded3, "123456");
    }
}
