//! A procedural macro crate providing the `#[serialization_test]` attribute.
//!
//! This crate is designed to reduce boilerplate when writing tests for types
//! that should serialize to and deserialize from a specific string structure.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream}, parse_macro_input, ItemFn,
    LitStr,
    Token,
};

/// Represents the possible arguments for the `#[serialization_test]` macro.
///
/// The macro can accept either a file path to a fixture or a raw
/// string literal representing the expected output.
enum MacroArgs {
    /// A relative path to a fixture file, e.g., `fixture = "path/to/file.xml"`.
    FixturePath(LitStr),
    /// The expected string result, e.g., `expected = "<tag>value</tag>"`.
    ExpectedResult(LitStr),
}

/// Parser implementation for `MacroArgs`.
///
/// This allows `syn` to parse the attribute's arguments from a token stream
/// into the `MacroArgs` enum. It expects a key-value pair format like
/// `key = "value"`.
impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: syn::Ident = input.parse()?;
        let _eq_token: Token![=] = input.parse()?;
        let value: LitStr = input.parse()?;

        if key == "fixture" {
            Ok(MacroArgs::FixturePath(value))
        } else if key == "expected" {
            Ok(MacroArgs::ExpectedResult(value))
        } else {
            Err(syn::Error::new(
                key.span(),
                "expected attribute `fixture` or `expected`",
            ))
        }
    }
}

/// Generates a pair of serialization and deserialization tests.
///
/// This attribute macro is attached to a "setup" function that returns an
/// instance of a type that implements `Serialize` and `Deserialize`. It then
/// generates two `#[test]` functions:
///
/// 1.  `serialize_<name>`: Asserts that serializing the instance from the setup
///     function matches the provided fixture/string.
/// 2.  `deserialize_<name>`: Asserts that deserializing the fixture/string
///     results in an instance equal to the one from the setup function.
///
/// # Arguments
///
/// * `fixture = "path/to/your/fixture.file"`: Use an external file.
/// * `expected = "<your><content/></your>"`: Use an inline string.
///
/// # Panics
///
/// The generated tests will panic if serialization, deserialization, or
/// canonicalization fails.
///
/// # Assumptions
///
/// This macro assumes the following helper functions are available in the scope
/// where the tests are generated:
///
/// * `serialize<T: Serialize>(value: &T) -> Result<String, _>`
/// * `deserialize<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, _>`
/// * `canonicalize(s: &str) -> Result<String, _>`
///
/// # Example
///
/// ```rust,ignore
/// #[serialization_test(fixture = "../tests/fixtures/detail.xml")]
/// fn setup_detail() -> Detail {
///     // ... return a Detail instance
/// }
/// // This will generate `serialize_detail()` and `deserialize_detail()` tests.
/// ```
#[proc_macro_attribute]
pub fn serialization_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);

    let setup_fn = parse_macro_input!(item as ItemFn);

    let setup_fn_name = &setup_fn.sig.ident;
    let setup_fn_name_str = setup_fn_name.to_string();

    let base_name = setup_fn_name_str
        .strip_prefix("setup_")
        .unwrap_or(&setup_fn_name_str);

    let serialize_test_name = format_ident!("serialize_{}", base_name);
    let deserialize_test_name = format_ident!("deserialize_{}", base_name);

    let return_type = match &setup_fn.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        syn::ReturnType::Default => {
            let msg = "function must have a return type to be used with #[serialization_test]";
            return syn::Error::new(setup_fn.sig.ident.span(), msg)
                .to_compile_error()
                .into();
        }
    };

    let expected_content_provider = match args {
        MacroArgs::FixturePath(path) => quote! { include_str!(#path) },
        MacroArgs::ExpectedResult(result_literal) => quote! { #result_literal },
    };

    let expanded = quote! {
        #setup_fn

        #[test]
        fn #serialize_test_name() {
            let instance = #setup_fn_name();
            let serialized = serialize(&instance)
                .expect("Failed to serialize instance");

            let canonicalized_output = canonicalize(&serialized)
                .expect("Failed to canonicalize serialized output");

            let fixture_content = #expected_content_provider;
            let expected_canonical = canonicalize(fixture_content)
                .expect("Failed to canonicalize fixture content");

            assert_eq!(canonicalized_output, expected_canonical, "Serialized output does not match fixture");
        }

        #[test]
        fn #deserialize_test_name() {
            let expected_instance = #setup_fn_name();

            let fixture_content = #expected_content_provider;
            let canonicalized_fixture = canonicalize(fixture_content)
                .expect("Failed to canonicalize fixture content");

            let deserialized: #return_type = deserialize(&canonicalized_fixture)
                .expect("Failed to deserialize fixture content");

            assert_eq!(deserialized, expected_instance, "Deserialized instance does not match setup instance");
        }
    };

    TokenStream::from(expanded)
}
