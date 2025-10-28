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
    let source = "{ int: 42, float: 3.14159 }";
    let root = document(source).expect("parse failed");
    let tbl = table(&root).expect("table failed");

    let int_node = tbl.get("int").expect("int not found");
    assert_eq!(uint64(int_node).expect("uint64 failed"), 42);

    let float_node = tbl.get("float").expect("float not found");
    let val = double(float_node).expect("double failed");
    assert!((val - std::f64::consts::PI).abs() < 0.00001);
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
