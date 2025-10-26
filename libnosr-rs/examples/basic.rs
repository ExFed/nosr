//! Basic example showing how to parse and navigate nosr documents.

use libnosr_rs::{document, double, tab, text, uint64, vec};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Simple scalar
    println!("=== Example 1: Simple Scalar ===");
    let source = r#""hello, world!""#;
    let node = document(source)?;
    println!("Text: {}", text(&node)?);

    // Example 2: Table
    println!("\n=== Example 2: Table ===");
    let source = r#"{
        name: Alice
        age: 30
        city: "San Francisco"
    }"#;
    let root = document(source)?;

    let name = tab(&root, "name")?;
    println!("Name: {}", text(&name)?);

    let age = tab(&root, "age")?;
    println!("Age: {}", uint64(&age)?);

    let city = tab(&root, "city")?;
    println!("City: {}", text(&city)?);

    // Example 3: Vector
    println!("\n=== Example 3: Vector ===");
    let source = "[apple, banana, cherry]";
    let root = document(source)?;

    println!("Fruits:");
    for i in 0..3 {
        let fruit = vec(&root, i)?;
        println!("  {}: {}", i, text(&fruit)?);
    }

    // Example 4: Nested structures
    println!("\n=== Example 4: Nested Structures ===");
    let source = r#"{
        person: {
            name: Bob
            age: 25
            hobbies: [coding, reading, hiking]
        }
        scores: [95.5, 87.3, 92.1]
    }"#;
    let root = document(source)?;

    let person = tab(&root, "person")?;
    let name = tab(&person, "name")?;
    println!("Person name: {}", text(&name)?);

    let hobbies = tab(&person, "hobbies")?;
    println!("Hobbies:");
    for i in 0..3 {
        let hobby = vec(&hobbies, i)?;
        println!("  - {}", text(&hobby)?);
    }

    let scores = tab(&root, "scores")?;
    println!("Scores:");
    for i in 0..3 {
        let score = vec(&scores, i)?;
        println!("  {}: {:.1}", i, double(&score)?);
    }

    // Example 5: Comments
    println!("\n=== Example 5: Comments ===");
    let source = r#"
        // This is a line comment
        {
            /* Block comments work too! */
            message: "Comments are ignored by the parser"
        }
    "#;
    let root = document(source)?;
    let message = tab(&root, "message")?;
    println!("Message: {}", text(&message)?);

    // Example 6: Escape sequences
    println!("\n=== Example 6: Escape Sequences ===");
    let source = r#"{
        quote: "She said \"Hello!\""
        newline: "Line 1\nLine 2"
        special: "Colon\: and bracket\["
    }"#;
    let root = document(source)?;

    let quote = tab(&root, "quote")?;
    println!("Quote: {}", text(&quote)?);

    let newline = tab(&root, "newline")?;
    println!("Newline: {}", text(&newline)?);

    let special = tab(&root, "special")?;
    println!("Special: {}", text(&special)?);

    Ok(())
}
