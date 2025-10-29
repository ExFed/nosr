//! Examples of pathological but valid nosr usage.
//!
//! This example demonstrates edge cases that are valid according to the nosr
//! specification but might be considered unusual or extreme in real-world usage.

use libnosr_rs::{document, double, table, text, uint64, vector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Empty structures
    println!("=== Example 1: Empty Structures ===");
    let empty_table = document("{}")?;
    let tbl = table(&empty_table)?;
    println!("Empty table has {} entries", tbl.len());

    let empty_vector = document("[]")?;
    let vec = vector(&empty_vector)?;
    println!("Empty vector has {} elements", vec.len());

    // Example 2: Deeply nested structures
    println!("\n=== Example 2: Deeply Nested Structures ===");
    let deep_source = r#"{
        level1: {
            level2: {
                level3: {
                    level4: {
                        level5: "You found me!"
                    }
                }
            }
        }
    }"#;
    let root = document(deep_source)?;
    let l1 = table(&root)?;
    let l2 = table(l1.get("level1").unwrap())?;
    let l3 = table(l2.get("level2").unwrap())?;
    let l4 = table(l3.get("level3").unwrap())?;
    let l5 = table(l4.get("level4").unwrap())?;
    let value = l5.get("level5").unwrap();
    println!("Deep value: {}", text(value)?);

    // Example 3: All escape sequences
    println!("\n=== Example 3: All Escape Sequences ===");
    let escape_source = r#""Special chars: \\ \n \t \r \: \" \[ \] \{ \}""#;
    let node = document(escape_source)?;
    println!("Escaped text: {:?}", text(&node)?);

    // Example 4: Unicode in keys and values
    println!("\n=== Example 4: Unicode ===");
    let unicode_source = r#"{
        世界: "World"
        🌍: "Earth"
        مرحبا: "Hello"
        Привет: "Hi"
    }"#;
    let root = document(unicode_source)?;
    let tbl = table(&root)?;
    println!("世界 = {}", text(tbl.get("世界").unwrap())?);
    println!("🌍 = {}", text(tbl.get("🌍").unwrap())?);
    println!("مرحبا = {}", text(tbl.get("مرحبا").unwrap())?);
    println!("Привет = {}", text(tbl.get("Привет").unwrap())?);

    // Example 5: Extreme whitespace
    println!("\n=== Example 5: Extreme Whitespace ===");
    let whitespace_source = r#"
    
    {
        
        key1    :    value1
        
        
        key2    :    value2
        
    }
    
    "#;
    let root = document(whitespace_source)?;
    let tbl = table(&root)?;
    println!("key1: {}", text(tbl.get("key1").unwrap())?);
    println!("key2: {}", text(tbl.get("key2").unwrap())?);

    // Example 6: Mixed delimiters
    println!("\n=== Example 6: Mixed Delimiters ===");
    let mixed_source = r#"[
        one,
        two
        three,
        four
    ]"#;
    let root = document(mixed_source)?;
    let vec = vector(&root)?;
    println!("Elements:");
    for (i, elem) in vec.iter().enumerate() {
        println!("  {}: {}", i, text(elem)?);
    }

    // Example 7: Quoted keys with special characters
    println!("\n=== Example 7: Quoted Keys with Special Characters ===");
    let special_keys = r#"{
        "key:with:colons": "value1"
        "key[with]brackets": "value2"
        "key{with}braces": "value3"
        "key,with,commas": "value4"
    }"#;
    let root = document(special_keys)?;
    let tbl = table(&root)?;
    println!(
        "'key:with:colons' = {}",
        text(tbl.get("key:with:colons").unwrap())?
    );
    println!(
        "'key[with]brackets' = {}",
        text(tbl.get("key[with]brackets").unwrap())?
    );
    println!(
        "'key{{with}}braces' = {}",
        text(tbl.get("key{with}braces").unwrap())?
    );
    println!(
        "'key,with,commas' = {}",
        text(tbl.get("key,with,commas").unwrap())?
    );

    // Example 8: Special numeric values
    println!("\n=== Example 8: Special Numeric Values ===");
    let special_nums = r#"{
        zero: 0
        max_u64: 18446744073709551615
        infinity: inf
        neg_infinity: -inf
        not_a_number: nan
    }"#;
    let root = document(special_nums)?;
    let tbl = table(&root)?;
    println!("zero = {}", uint64(tbl.get("zero").unwrap())?);
    println!("max_u64 = {}", uint64(tbl.get("max_u64").unwrap())?);

    let inf = double(tbl.get("infinity").unwrap())?;
    println!("infinity is infinite: {}", inf.is_infinite());

    let neg_inf = double(tbl.get("neg_infinity").unwrap())?;
    println!(
        "neg_infinity is negative infinite: {}",
        neg_inf.is_infinite() && neg_inf.is_sign_negative()
    );

    let nan = double(tbl.get("not_a_number").unwrap())?;
    println!("not_a_number is NaN: {}", nan.is_nan());

    // Example 9: Empty string
    println!("\n=== Example 9: Empty String ===");
    let empty_str = r#""""#;
    let node = document(empty_str)?;
    let str_val = text(&node)?;
    println!("Empty string length: {}", str_val.len());
    println!("Is empty: {}", str_val.is_empty());

    // Example 10: Complex nested mix
    println!("\n=== Example 10: Complex Nested Mix ===");
    let complex = r#"{
        users: [
            { id: 1, name: "Alice", tags: [admin, user] }
            { id: 2, name: "Bob", tags: [user] }
            { id: 3, name: "Charlie", tags: [user, guest] }
        ]
        metadata: {
            total: 3
            active: 2
            settings: {
                theme: dark
                lang: en
            }
        }
    }"#;
    let root = document(complex)?;
    let tbl = table(&root)?;

    let users = vector(tbl.get("users").unwrap())?;
    println!("Number of users: {}", users.len());

    let first_user = table(&users[0])?;
    println!("First user: {}", text(first_user.get("name").unwrap())?);

    let tags = vector(first_user.get("tags").unwrap())?;
    println!("First user tags: {} tags", tags.len());

    let metadata = table(tbl.get("metadata").unwrap())?;
    let settings = table(metadata.get("settings").unwrap())?;
    println!("Theme: {}", text(settings.get("theme").unwrap())?);

    // Example 11: Multiple trailing delimiters
    println!("\n=== Example 11: Multiple Trailing Delimiters ===");
    let trailing = "[a, b, c,,,]";
    let root = document(trailing)?;
    let vec = vector(&root)?;
    println!("Vector with trailing commas has {} elements", vec.len());

    // Example 12: Single element structures
    println!("\n=== Example 12: Single Element Structures ===");
    let single_vec = "[only_one]";
    let root = document(single_vec)?;
    let vec = vector(&root)?;
    println!("Single element vector: {}", text(&vec[0])?);

    let single_table = "{ only_key: only_value }";
    let root = document(single_table)?;
    let tbl = table(&root)?;
    println!(
        "Single element table: {}",
        text(tbl.get("only_key").unwrap())?
    );

    // Example 13: Deeply nested vectors
    println!("\n=== Example 13: Deeply Nested Vectors ===");
    let deep_vec = "[[[[42]]]]";
    let root = document(deep_vec)?;
    let v1 = vector(&root)?;
    let v2 = vector(&v1[0])?;
    let v3 = vector(&v2[0])?;
    let v4 = vector(&v3[0])?;
    println!("Deeply nested value: {}", uint64(&v4[0])?);

    println!("\n=== All Examples Completed Successfully! ===");

    Ok(())
}
