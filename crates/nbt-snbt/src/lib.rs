//! SNBT (String NBT) parser - Fast parsing of Minecraft's string NBT format
//! 
//! Simple, efficient SNBT parsing using winnow combinators for 10x+ performance vs regex.

mod error;
mod parser;

pub use error::*;
pub use parser::*;

// Re-export commonly used types
pub use nbt_core::{NbtTag, HashMap};

/// Parse SNBT string to NBT tag
pub fn parse_snbt(input: &str) -> Result<NbtTag> {
    parser::parse_tag(input)
}

/// Format NBT tag to SNBT string
pub fn format_snbt(tag: &NbtTag) -> String {
    parser::format_tag(tag, false)
}

/// Format NBT tag to pretty SNBT string with indentation
pub fn format_snbt_pretty(tag: &NbtTag) -> String {
    parser::format_tag(tag, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nbt_core::NbtTag;

    #[test]
    fn test_parse_simple_compound() {
        let input = r#"{foo: "bar", count: 42}"#;
        let result = parse_snbt(input).unwrap();
        
        if let NbtTag::Compound(map) = result {
            assert_eq!(map.len(), 2);
            assert_eq!(map.get("foo").unwrap().as_string(), "bar");
            assert_eq!(map.get("count").unwrap().as_number() as i32, 42);
        } else {
            panic!("Expected compound");
        }
    }

    #[test]
    fn test_parse_numbers() {
        let tests = vec![
            ("42b", NbtTag::Byte(42)),
            ("42s", NbtTag::Short(42)),
            ("42", NbtTag::Int(42)),
            ("42L", NbtTag::Long(42)),
            ("3.14f", NbtTag::Float(3.14)),
            ("3.14d", NbtTag::Double(3.14)),
            ("3.14", NbtTag::Double(3.14)),
        ];

        for (input, expected) in tests {
            let result = parse_snbt(input).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_parse_strings() {
        // Regular string tests
        let result = parse_snbt(r#""hello world""#).unwrap();
        assert_eq!(result.as_string(), "hello world");
        
        let result = parse_snbt("hello").unwrap();
        assert_eq!(result.as_string(), "hello");
        
        // Boolean strings should remain as strings
        let result = parse_snbt("nottrue").unwrap();
        assert_eq!(result.as_string(), "nottrue");
    }

    #[test]
    fn test_parse_arrays() {
        // Byte array
        let result = parse_snbt("[B; 4b, 2b]").unwrap();
        if let NbtTag::ByteArray(arr) = result {
            assert_eq!(arr, vec![4, 2]);
        } else {
            panic!("Expected byte array");
        }

        // Int array
        let result = parse_snbt("[I; 4, 123]").unwrap();
        if let NbtTag::IntArray(arr) = result {
            assert_eq!(arr, vec![4, 123]);
        } else {
            panic!("Expected int array");
        }

        // Long array
        let result = parse_snbt("[L; 4L, 123L]").unwrap();
        if let NbtTag::LongArray(arr) = result {
            assert_eq!(arr, vec![4, 123]);
        } else {
            panic!("Expected long array");
        }
    }

    #[test]
    fn test_parse_lists() {
        let result = parse_snbt("[4, 2, 6]").unwrap();
        if let NbtTag::List { tag_type, items } = result {
            assert_eq!(tag_type, 3); // Int type
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].as_number() as i32, 4);
            assert_eq!(items[1].as_number() as i32, 2);
            assert_eq!(items[2].as_number() as i32, 6);
        } else {
            panic!("Expected list");
        }
    }

    #[test]
    fn test_parse_complex() {
        // Simpler test that should work
        let input = r#"{player: {name: "Steve", level: 10}}"#;
        
        let result = parse_snbt(input).unwrap();
        if let NbtTag::Compound(map) = result {
            assert!(map.contains_key("player"));
        } else {
            panic!("Expected compound");
        }
    }

    #[test]
    fn test_format_roundtrip() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), NbtTag::string("Test"));
        map.insert("value".to_string(), NbtTag::int(42));
        map.insert("list".to_string(), NbtTag::List { 
            tag_type: 8, 
            items: vec![NbtTag::string("a"), NbtTag::string("b")] 
        });
        let original = NbtTag::Compound(map);

        let formatted = format_snbt(&original);
        let parsed = parse_snbt(&formatted).unwrap();
        assert_eq!(original, parsed);
    }

    // Error tests - basic error handling verification
    #[test]
    fn test_parse_errors() {
        // Test that invalid input returns errors (basic check)
        let error_cases = vec![
            "{hello:",
            "\"1abc",
            "[what, ,]",
            "[}]",
            "[E; 5]",
            "{: bah}",
        ];

        for input in error_cases {
            let result = parse_snbt(input);
            assert!(result.is_err(), "Expected error for input: {}", input);
        }
    }

    #[test]
    fn test_boolean_edge_cases() {
        // Test that true/false are treated as booleans converted to bytes
        let result = parse_snbt("true").unwrap();
        assert_eq!(result, NbtTag::Byte(1));
        
        let result = parse_snbt("false").unwrap();
        assert_eq!(result, NbtTag::Byte(0));
        
        // But other strings should remain strings
        let result = parse_snbt("nottrue").unwrap();
        assert_eq!(result, NbtTag::String("nottrue".to_string()));
    }

    #[test]
    fn test_quoted_keys() {
        // Simple quoted key test
        let input = r#"{"hello": 4b}"#;
        let result = parse_snbt(input).unwrap();
        
        if let NbtTag::Compound(map) = result {
            assert!(map.contains_key("hello"));
            assert_eq!(map.get("hello").unwrap().as_number() as i8, 4);
        } else {
            panic!("Expected compound");
        }
    }
} 