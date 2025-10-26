//! Integration tests for libnosr-rs
//!
//! These tests verify the parser against examples from the nosr specification.

use libnosr_rs::{document, double, tab, text, uint64, vec};

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

    let elem0 = vec(&root, 0).expect("vec[0] failed");
    assert_eq!(text(&elem0).expect("text failed"), "some");

    let elem1 = vec(&root, 1).expect("vec[1] failed");
    assert_eq!(text(&elem1).expect("text failed"), "kind");

    let elem2 = vec(&root, 2).expect("vec[2] failed");
    assert_eq!(text(&elem2).expect("text failed"), "of");

    let elem3 = vec(&root, 3).expect("vec[3] failed");
    assert_eq!(text(&elem3).expect("text failed"), "vector");
}

#[test]
fn test_simple_table() {
    let source = r#"{
        letters: abcd
        numbers: 1234
    }"#;

    let root = document(source).expect("parse failed");

    let letters = tab(&root, "letters").expect("tab failed");
    assert_eq!(text(&letters).expect("text failed"), "abcd");

    let numbers = tab(&root, "numbers").expect("tab failed");
    assert_eq!(text(&numbers).expect("text failed"), "1234");
}

#[test]
fn test_table_with_escapes() {
    let source = r#"{
        "escape\:me": "have a double quote\:\""
    }"#;

    let root = document(source).expect("parse failed");
    let value = tab(&root, "escape:me").expect("tab failed");
    assert_eq!(
        text(&value).expect("text failed"),
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
    let person = tab(&root, "person").expect("tab failed");
    let name = tab(&person, "name").expect("tab failed");
    assert_eq!(text(&name).expect("text failed"), "Alice");

    let age = tab(&person, "age").expect("tab failed");
    assert_eq!(uint64(&age).expect("uint64 failed"), 30);
}

#[test]
fn test_nested_vector() {
    let source = "[[1, 2], [3, 4]]";

    let root = document(source).expect("parse failed");
    let first = vec(&root, 0).expect("vec failed");
    let second = vec(&root, 1).expect("vec failed");

    let first_0 = vec(&first, 0).expect("vec failed");
    assert_eq!(uint64(&first_0).expect("uint64 failed"), 1);

    let first_1 = vec(&first, 1).expect("vec failed");
    assert_eq!(uint64(&first_1).expect("uint64 failed"), 2);

    let second_0 = vec(&second, 0).expect("vec failed");
    assert_eq!(uint64(&second_0).expect("uint64 failed"), 3);

    let second_1 = vec(&second, 1).expect("vec failed");
    assert_eq!(uint64(&second_1).expect("uint64 failed"), 4);
}

#[test]
fn test_numbers() {
    let source = "{ int: 42, float: 3.14159 }";
    let root = document(source).expect("parse failed");

    let int_node = tab(&root, "int").expect("tab failed");
    assert_eq!(uint64(&int_node).expect("uint64 failed"), 42);

    let float_node = tab(&root, "float").expect("tab failed");
    let val = double(&float_node).expect("double failed");
    assert!((val - 3.14159).abs() < 0.00001);
}

#[test]
fn test_comments() {
    let source = r#"
    // This is a line comment
    {
        /* block comment */
        key: value // another comment
    }
    "#;

    let root = document(source).expect("parse failed");
    eprintln!("Root span: {:?}", root.span());
    eprintln!("Root raw: {:?}", root.raw());
    eprintln!("Starts with brace: {}", root.raw().trim().starts_with('{'));
    let value = tab(&root, "key").expect("tab failed");
    assert_eq!(text(&value).expect("text failed"), "value");
}

#[test]
fn test_multiline_vector() {
    let source = r#"[
        one
        two
        three
    ]"#;

    let root = document(source).expect("parse failed");

    let elem0 = vec(&root, 0).expect("vec failed");
    assert_eq!(text(&elem0).expect("text failed"), "one");

    let elem1 = vec(&root, 1).expect("vec failed");
    assert_eq!(text(&elem1).expect("text failed"), "two");

    let elem2 = vec(&root, 2).expect("vec failed");
    assert_eq!(text(&elem2).expect("text failed"), "three");
}

#[test]
fn test_vector_with_trailing_delimiter() {
    let source = "[one, two, three,]";
    let root = document(source).expect("parse failed");

    let elem0 = vec(&root, 0).expect("vec failed");
    assert_eq!(text(&elem0).expect("text failed"), "one");

    let elem2 = vec(&root, 2).expect("vec failed");
    assert_eq!(text(&elem2).expect("text failed"), "three");
}

#[test]
fn test_table_with_semicolons() {
    let source = "{ a: 1; b: 2; c: 3 }";
    let root = document(source).expect("parse failed");

    let a = tab(&root, "a").expect("tab failed");
    assert_eq!(uint64(&a).expect("uint64 failed"), 1);

    let b = tab(&root, "b").expect("tab failed");
    assert_eq!(uint64(&b).expect("uint64 failed"), 2);

    let c = tab(&root, "c").expect("tab failed");
    assert_eq!(uint64(&c).expect("uint64 failed"), 3);
}
