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

    let vector = vec(&root).expect("vec failed");
    assert_eq!(vector.len(), 4);

    assert_eq!(text(&vector[0]).expect("text failed"), "some");
    assert_eq!(text(&vector[1]).expect("text failed"), "kind");
    assert_eq!(text(&vector[2]).expect("text failed"), "of");
    assert_eq!(text(&vector[3]).expect("text failed"), "vector");
}

#[test]
fn test_simple_table() {
    let source = r#"{
        letters: abcd
        numbers: 1234
    }"#;

    let root = document(source).expect("parse failed");
    let table = tab(&root).expect("tab failed");

    let letters = table.get("letters").expect("letters not found");
    assert_eq!(text(letters).expect("text failed"), "abcd");

    let numbers = table.get("numbers").expect("numbers not found");
    assert_eq!(text(numbers).expect("text failed"), "1234");
}

#[test]
fn test_table_with_escapes() {
    let source = r#"{
        "escape\:me": "have a double quote\:\""
    }"#;

    let root = document(source).expect("parse failed");
    let table = tab(&root).expect("tab failed");
    let value = table.get("escape:me").expect("key not found");
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
    let root_table = tab(&root).expect("tab failed");
    let person = root_table.get("person").expect("person not found");
    let person_table = tab(person).expect("tab failed");

    let name = person_table.get("name").expect("name not found");
    assert_eq!(text(name).expect("text failed"), "Alice");

    let age = person_table.get("age").expect("age not found");
    assert_eq!(uint64(age).expect("uint64 failed"), 30);
}

#[test]
fn test_nested_vector() {
    let source = "[[1, 2], [3, 4]]";

    let root = document(source).expect("parse failed");
    let outer_vec = vec(&root).expect("vec failed");

    let first = &outer_vec[0];
    let second = &outer_vec[1];

    let first_vec = vec(first).expect("vec failed");
    assert_eq!(uint64(&first_vec[0]).expect("uint64 failed"), 1);
    assert_eq!(uint64(&first_vec[1]).expect("uint64 failed"), 2);

    let second_vec = vec(second).expect("vec failed");
    assert_eq!(uint64(&second_vec[0]).expect("uint64 failed"), 3);
    assert_eq!(uint64(&second_vec[1]).expect("uint64 failed"), 4);
}

#[test]
fn test_numbers() {
    let source = "{ int: 42, float: 3.14159 }";
    let root = document(source).expect("parse failed");
    let table = tab(&root).expect("tab failed");

    let int_node = table.get("int").expect("int not found");
    assert_eq!(uint64(int_node).expect("uint64 failed"), 42);

    let float_node = table.get("float").expect("float not found");
    let val = double(float_node).expect("double failed");
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
    let table = tab(&root).expect("tab failed");
    let value = table.get("key").expect("key not found");
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
    let vector = vec(&root).expect("vec failed");

    assert_eq!(text(&vector[0]).expect("text failed"), "one");
    assert_eq!(text(&vector[1]).expect("text failed"), "two");
    assert_eq!(text(&vector[2]).expect("text failed"), "three");
}

#[test]
fn test_vector_with_trailing_delimiter() {
    let source = "[one, two, three,]";
    let root = document(source).expect("parse failed");
    let vector = vec(&root).expect("vec failed");

    assert_eq!(text(&vector[0]).expect("text failed"), "one");
    assert_eq!(text(&vector[2]).expect("text failed"), "three");
}

#[test]
fn test_table_with_semicolons() {
    let source = "{ a: 1; b: 2; c: 3 }";
    let root = document(source).expect("parse failed");
    let table = tab(&root).expect("tab failed");

    let a = table.get("a").expect("a not found");
    assert_eq!(uint64(a).expect("uint64 failed"), 1);

    let b = table.get("b").expect("b not found");
    assert_eq!(uint64(b).expect("uint64 failed"), 2);

    let c = table.get("c").expect("c not found");
    assert_eq!(uint64(c).expect("uint64 failed"), 3);
}
