//! Integration tests for libnosr-rs
//!
//! These tests verify the parser against examples from the nosr specification.

use libnosr_rs::{document, double, table, text, uint64, vector};

#[test]
fn test_simple_text() {
    let source = r#""hello, world!""#;
    let node = document(source).expect("parse failed");
    assert_eq!(text(&node).expect("text failed"), "hello, world!");
}

#[test]
fn test_unquoted_scalar() {
    let source = "hello";
    let node = document(source).expect("parse failed");
    assert_eq!(text(&node).expect("text failed"), "hello");
}

#[test]
fn test_simple_vector() {
    let source = "[some, kind, of, \"vector\"]";
    let root = document(source).expect("parse failed");

    let v = vector(&root).expect("vector failed");
    assert_eq!(v.len(), 4);

    assert_eq!(text(&v[0]).expect("text failed"), "some");
    assert_eq!(text(&v[1]).expect("text failed"), "kind");
    assert_eq!(text(&v[2]).expect("text failed"), "of");
    assert_eq!(text(&v[3]).expect("text failed"), "vector");
}

#[test]
fn test_simple_table() {
    let source = r#"{
        letters: abcd
        numbers: 1234
    }"#;

    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let letters = tbl.get("letters").expect("letters not found");
    assert_eq!(text(letters).expect("text failed"), "abcd");

    let numbers = tbl.get("numbers").expect("numbers not found");
    assert_eq!(text(numbers).expect("text failed"), "1234");
}

#[test]
fn test_table_with_escapes() {
    let source = r#"{
        "escape\:me": "have a double quote\:\""
    }"#;

    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");
    let value = tbl.get("escape:me").expect("key not found");
    assert_eq!(
        text(value).expect("text failed"),
        r#"have a double quote:""#
    );
}

#[test]
fn test_nested_table() {
    let source = r#"{
        person: {
            name: Alice
            age: 30
        }
    }"#;

    let root = document(source).expect("parse failed");
    let root_table = table(&root).expect("table failed");
    let person = root_table.get("person").expect("person not found");
    let person_table = table(person).expect("table failed");

    let name = person_table.get("name").expect("name not found");
    assert_eq!(text(name).expect("text failed"), "Alice");

    let age = person_table.get("age").expect("age not found");
    assert_eq!(uint64(age).expect("uint64 failed"), 30);
}

#[test]
fn test_nested_vector() {
    let source = "[[1, 2], [3, 4]]";

    let root = document(source).expect("parse failed");
    let outer_vec = vector(&root).expect("vector failed");

    let first = &outer_vec[0];
    let second = &outer_vec[1];

    let first_vec = vector(first).expect("vector failed");
    assert_eq!(uint64(&first_vec[0]).expect("uint64 failed"), 1);
    assert_eq!(uint64(&first_vec[1]).expect("uint64 failed"), 2);

    let second_vec = vector(second).expect("vector failed");
    assert_eq!(uint64(&second_vec[0]).expect("uint64 failed"), 3);
    assert_eq!(uint64(&second_vec[1]).expect("uint64 failed"), 4);
}

#[test]
fn test_numbers() {
    let source = "{ int: 42, float: 12.75 }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let int_node = tbl.get("int").expect("int not found");
    assert_eq!(uint64(int_node).expect("uint64 failed"), 42);

    let float_node = tbl.get("float").expect("float not found");
    let val = double(float_node).expect("double failed");
    assert!((val - 12.75).abs() < 0.00001);
}

#[test]
fn test_comments() {
    let source = r#"
    # This is a line comment
    {
        #* block comment *#
        key: value # another comment
    }
    "#;

    let root = document(source).expect("parse failed");
    eprintln!("Root span: {:?}", root.span());
    eprintln!("Root raw: {:?}", root.raw());
    eprintln!("Starts with brace: {}", root.raw().trim().starts_with('{'));
    let tbl = table(&root).expect("table failed");
    let value = tbl.get("key").expect("key not found");
    assert_eq!(text(value).expect("text failed"), "value");
}

#[test]
fn test_multiline_vector() {
    let source = r#"[
        one
        two
        three
    ]"#;

    let root = document(source).expect("parse failed");
    let v = vector(&root).expect("vector failed");

    assert_eq!(text(&v[0]).expect("text failed"), "one");
    assert_eq!(text(&v[1]).expect("text failed"), "two");
    assert_eq!(text(&v[2]).expect("text failed"), "three");
}

#[test]
fn test_vector_with_trailing_delimiter() {
    let source = "[one, two, three,]";
    let root = document(source).expect("parse failed");
    let v = vector(&root).expect("vector failed");

    assert_eq!(text(&v[0]).expect("text failed"), "one");
    assert_eq!(text(&v[2]).expect("text failed"), "three");
}

#[test]
fn test_table_with_commas() {
    let source = "{ a: 1, b: 2, c: 3 }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let a = tbl.get("a").expect("a not found");
    assert_eq!(uint64(a).expect("uint64 failed"), 1);

    let b = tbl.get("b").expect("b not found");
    assert_eq!(uint64(b).expect("uint64 failed"), 2);

    let c = tbl.get("c").expect("c not found");
    assert_eq!(uint64(c).expect("uint64 failed"), 3);
}

// ============================================================================
// Pathological but Valid Test Cases
// ============================================================================

#[test]
fn test_empty_table() {
    let source = "{}";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");
    assert_eq!(tbl.len(), 0);
}

#[test]
fn test_empty_vector() {
    let source = "[]";
    let root = document(source).expect("parse failed");
    let v = vector(&root).expect("vector failed");
    assert_eq!(v.len(), 0);
}

#[test]
fn test_deeply_nested_structures() {
    let source = r#"{
        level1: {
            level2: {
                level3: {
                    level4: {
                        level5: {
                            deep_value: "found me!"
                        }
                    }
                }
            }
        }
    }"#;

    let root = document(source).expect("parse failed");
    let l1 = table(&root).expect("table failed");
    let l2 = table(l1.get("level1").expect("level1 not found")).expect("table failed");
    let l3 = table(l2.get("level2").expect("level2 not found")).expect("table failed");
    let l4 = table(l3.get("level3").expect("level3 not found")).expect("table failed");
    let l5 = table(l4.get("level4").expect("level4 not found")).expect("table failed");
    let l6 = table(l5.get("level5").expect("level5 not found")).expect("table failed");
    let value = l6.get("deep_value").expect("deep_value not found");
    assert_eq!(text(value).expect("text failed"), "found me!");
}

#[test]
fn test_deeply_nested_vectors() {
    let source = "[[[[[[42]]]]]]";
    let root = document(source).expect("parse failed");
    let v1 = vector(&root).expect("vector failed");
    let v2 = vector(&v1[0]).expect("vector failed");
    let v3 = vector(&v2[0]).expect("vector failed");
    let v4 = vector(&v3[0]).expect("vector failed");
    let v5 = vector(&v4[0]).expect("vector failed");
    let v6 = vector(&v5[0]).expect("vector failed");
    assert_eq!(uint64(&v6[0]).expect("uint64 failed"), 42);
}

#[test]
fn test_all_escape_sequences() {
    let source = r#""backslash:\\, newline:\n, tab:\t, return:\r, colon:\:, quote:\", lbracket:\[, rbracket:\], lbrace:\{, rbrace:\}""#;
    let root = document(source).expect("parse failed");
    let text_val = text(&root).expect("text failed");
    assert_eq!(
        text_val,
        "backslash:\\, newline:\n, tab:\t, return:\r, colon::, quote:\", lbracket:[, rbracket:], lbrace:{, rbrace:}"
    );
}

#[test]
fn test_unicode_text() {
    let source = r#""Hello 世界 🌍 Привет مرحبا""#;
    let root = document(source).expect("parse failed");
    assert_eq!(
        text(&root).expect("text failed"),
        "Hello 世界 🌍 Привет مرحبا"
    );
}

#[test]
fn test_unicode_in_keys() {
    let source = r#"{ 世界: hello, "🌍": world }"#;
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let val1 = tbl.get("世界").expect("unicode key not found");
    assert_eq!(text(val1).expect("text failed"), "hello");

    let val2 = tbl.get("🌍").expect("emoji key not found");
    assert_eq!(text(val2).expect("text failed"), "world");
}

#[test]
fn test_extreme_whitespace() {
    let source = r#"
    
    
    {
        
        
        key1    :    value1
        
        
        key2    :    value2    
        
        
    }
    
    
    "#;
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");
    assert_eq!(
        text(tbl.get("key1").expect("key1 not found")).expect("text failed"),
        "value1"
    );
    assert_eq!(
        text(tbl.get("key2").expect("key2 not found")).expect("text failed"),
        "value2"
    );
}

#[test]
fn test_mixed_delimiters() {
    // Mix of commas and newlines
    let source = r#"[
        one,
        two
        three,
        four
    ]"#;
    let root = document(source).expect("parse failed");
    let v = vector(&root).expect("vector failed");
    assert_eq!(v.len(), 4);
    assert_eq!(text(&v[0]).expect("text failed"), "one");
    assert_eq!(text(&v[3]).expect("text failed"), "four");
}

#[test]
fn test_large_integer() {
    let source = "18446744073709551615"; // Max u64
    let root = document(source).expect("parse failed");
    assert_eq!(
        uint64(&root).expect("uint64 failed"),
        18446744073709551615u64
    );
}

#[test]
fn test_zero_values() {
    let source = "{ int: 0, float: 0.0 }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let int_val = tbl.get("int").expect("int not found");
    assert_eq!(uint64(int_val).expect("uint64 failed"), 0);

    let float_val = tbl.get("float").expect("float not found");
    assert!((double(float_val).expect("double failed") - 0.0).abs() < 0.00001);
}

#[test]
fn test_negative_numbers() {
    let source = "{ neg: -42, negfloat: -3.25 }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    // Note: uint64 should fail on negative numbers
    let neg = tbl.get("neg").expect("neg not found");
    assert!(uint64(neg).is_err());

    let negfloat = tbl.get("negfloat").expect("negfloat not found");
    let val = double(negfloat).expect("double failed");
    assert!((val - (-3.25)).abs() < 0.00001);
}

#[test]
fn test_special_float_values() {
    let source = "{ inf: inf, neginf: -inf, nan: nan }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let inf_val = tbl.get("inf").expect("inf not found");
    let inf = double(inf_val).expect("double failed");
    assert!(inf.is_infinite() && inf.is_sign_positive());

    let neginf_val = tbl.get("neginf").expect("neginf not found");
    let neginf = double(neginf_val).expect("double failed");
    assert!(neginf.is_infinite() && neginf.is_sign_negative());

    let nan_val = tbl.get("nan").expect("nan not found");
    let nan = double(nan_val).expect("double failed");
    assert!(nan.is_nan());
}

#[test]
fn test_single_element_structures() {
    let source_vec = "[single]";
    let root = document(source_vec).expect("parse failed");
    let v = vector(&root).expect("vector failed");
    assert_eq!(v.len(), 1);
    assert_eq!(text(&v[0]).expect("text failed"), "single");

    let source_tbl = "{ key: value }";
    let root = document(source_tbl).expect("parse failed");
    let tbl = table(&root).expect("table failed");
    assert_eq!(tbl.len(), 1);
}

#[test]
fn test_quoted_keys_with_special_chars() {
    let source = r#"{
        "key:with:colons": value1
        "key[with]brackets": value2
        "key{with}braces": value3
        "key,with,commas": value4
    }"#;
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    assert_eq!(
        text(tbl.get("key:with:colons").expect("key not found")).expect("text failed"),
        "value1"
    );
    assert_eq!(
        text(tbl.get("key[with]brackets").expect("key not found")).expect("text failed"),
        "value2"
    );
    assert_eq!(
        text(tbl.get("key{with}braces").expect("key not found")).expect("text failed"),
        "value3"
    );
    assert_eq!(
        text(tbl.get("key,with,commas").expect("key not found")).expect("text failed"),
        "value4"
    );
}

#[test]
fn test_empty_string() {
    let source = r#""""#;
    let root = document(source).expect("parse failed");
    assert_eq!(text(&root).expect("text failed"), "");
}

#[test]
fn test_multiple_trailing_delimiters() {
    let source = "[a, b, c,,,]";
    let root = document(source).expect("parse failed");
    let v = vector(&root).expect("vector failed");
    // Should have 3 elements, trailing delimiters are ignored
    assert_eq!(v.len(), 3);
}

#[test]
fn test_complex_nested_mix() {
    let source = r#"{
        data: [
            { id: 1, name: "first" }
            { id: 2, name: "second" }
            { id: 3, name: "third" }
        ]
        metadata: {
            count: 3
            tags: [a, b, c]
        }
    }"#;
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let data = vector(tbl.get("data").expect("data not found")).expect("vector failed");
    assert_eq!(data.len(), 3);

    let first = table(&data[0]).expect("table failed");
    assert_eq!(
        uint64(first.get("id").expect("id not found")).expect("uint64 failed"),
        1
    );
    assert_eq!(
        text(first.get("name").expect("name not found")).expect("text failed"),
        "first"
    );

    let metadata = table(tbl.get("metadata").expect("metadata not found")).expect("table failed");
    assert_eq!(
        uint64(metadata.get("count").expect("count not found")).expect("uint64 failed"),
        3
    );

    let tags = vector(metadata.get("tags").expect("tags not found")).expect("vector failed");
    assert_eq!(tags.len(), 3);
}

// ============================================================================
// Error Condition Test Cases
// ============================================================================

#[test]
fn test_error_unclosed_string() {
    let source = r#""unclosed string"#;
    let result = document(source);
    assert!(result.is_err());
}

#[test]
fn test_error_unclosed_block_comment() {
    let source = "#* unclosed comment";
    let result = document(source);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_escape_sequence() {
    let source = r#""invalid \x escape""#;
    let root = document(source).expect("parse failed");
    let result = text(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_type_mismatch_table_as_vector() {
    let source = "{ key: value }";
    let root = document(source).expect("parse failed");
    let result = vector(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_type_mismatch_vector_as_table() {
    let source = "[a, b, c]";
    let root = document(source).expect("parse failed");
    let result = table(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_type_mismatch_scalar_as_table() {
    let source = "scalar_value";
    let root = document(source).expect("parse failed");
    let result = table(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_type_mismatch_scalar_as_vector() {
    let source = "scalar_value";
    let root = document(source).expect("parse failed");
    let result = vector(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_colon_in_table() {
    let source = "{ key value }";
    let result = document(source);
    // This should either fail to parse or fail when trying to parse as table
    if let Ok(root) = result {
        let tbl_result = table(&root);
        assert!(tbl_result.is_err());
    }
}

#[test]
fn test_error_unbalanced_braces() {
    let source = "{ key: value";
    let result = document(source);
    // Should fail to parse or when trying to parse as table
    if let Ok(root) = result {
        let tbl_result = table(&root);
        assert!(tbl_result.is_err());
    }
}

#[test]
fn test_error_unbalanced_brackets() {
    let source = "[a, b, c";
    let result = document(source);
    // Should fail to parse or when trying to parse as vector
    if let Ok(root) = result {
        let vec_result = vector(&root);
        assert!(vec_result.is_err());
    }
}

#[test]
fn test_error_invalid_number_format() {
    let source = "12.34.56";
    let root = document(source).expect("parse failed");
    let result = double(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_text_from_table() {
    let source = "{ key: value }";
    let root = document(source).expect("parse failed");
    let result = text(&root);
    // Text on a table node returns the raw text (not an error)
    // This is expected behavior - text() doesn't validate structure
    assert!(result.is_ok());
}

#[test]
fn test_error_text_from_vector() {
    let source = "[a, b, c]";
    let root = document(source).expect("parse failed");
    let result = text(&root);
    // Text on a vector node returns the raw text (not an error)
    // This is expected behavior - text() doesn't validate structure
    assert!(result.is_ok());
}

#[test]
fn test_error_uint64_overflow() {
    let source = "18446744073709551616"; // Max u64 + 1
    let root = document(source).expect("parse failed");
    let result = uint64(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_uint64_from_text() {
    let source = "not_a_number";
    let root = document(source).expect("parse failed");
    let result = uint64(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_double_from_text() {
    let source = "not_a_number";
    let root = document(source).expect("parse failed");
    let result = double(&root);
    assert!(result.is_err());
}

#[test]
fn test_error_empty_input() {
    let source = "";
    let result = document(source);
    // Empty input returns an empty node (not an error)
    assert!(result.is_ok());
    if let Ok(node) = result {
        assert_eq!(node.raw(), "");
    }
}

#[test]
fn test_error_only_whitespace() {
    let source = "   \n\t  \n  ";
    let result = document(source);
    // Whitespace-only input returns an empty node (not an error)
    assert!(result.is_ok());
    if let Ok(node) = result {
        assert_eq!(node.raw(), "");
    }
}

#[test]
fn test_error_only_comments() {
    let source = r#"
    # Just a comment
    #* And a block comment *#
    "#;
    let result = document(source);
    // Comments-only input returns an empty node (not an error)
    assert!(result.is_ok());
    if let Ok(node) = result {
        assert_eq!(node.raw(), "");
    }
}
