//! Basic example showing how to parse and navigate nosr documents.

use libnosr::{document, double, table, text, uint64, vector};

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
    let tbl = table(&root)?;

    let name = tbl.get("name").expect("name not found");
    println!("Name: {}", text(name)?);

    let age = tbl.get("age").expect("age not found");
    println!("Age: {}", uint64(age)?);

    let city = tbl.get("city").expect("city not found");
    println!("City: {}", text(city)?);

    // Example 3: Vector
    println!("\n=== Example 3: Vector ===");
    let source = "[apple, banana, cherry]";
    let root = document(source)?;
    let fruits = vector(&root)?;

    println!("Fruits:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", i, text(fruit)?);
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
    let root_table = table(&root)?;

    let person = root_table.get("person").expect("person not found");
    let person_table = table(person)?;
    let name = person_table.get("name").expect("name not found");
    println!("Person name: {}", text(name)?);

    let hobbies = person_table.get("hobbies").expect("hobbies not found");
    let hobbies_vec = vector(hobbies)?;
    println!("Hobbies:");
    for hobby in &hobbies_vec {
        println!("  - {}", text(hobby)?);
    }

    let scores = root_table.get("scores").expect("scores not found");
    let scores_vec = vector(scores)?;
    println!("Scores:");
    for (i, score) in scores_vec.iter().enumerate() {
        println!("  {}: {:.1}", i, double(score)?);
    }

    // Example 5: Comments
    println!("\n=== Example 5: Comments ===");
    let source = r#"
        # This is a line comment
        {
            #* Block comments work too! *#
            message: "Comments are ignored by the parser"
        }
    "#;
    let root = document(source)?;
    let tbl = table(&root)?;
    let message = tbl.get("message").expect("message not found");
    println!("Message: {}", text(message)?);

    // Example 6: Escape sequences
    println!("\n=== Example 6: Escape Sequences ===");
    let source = r#"{
        quote: "She said \"Hello!\""
        newline: "Line 1\nLine 2"
        special: "Colon\: and bracket\["
    }"#;
    let root = document(source)?;
    let tbl = table(&root)?;

    let quote = tbl.get("quote").expect("quote not found");
    println!("Quote: {}", text(quote)?);

    let newline = tbl.get("newline").expect("newline not found");
    println!("Newline: {}", text(newline)?);

    let special = tbl.get("special").expect("special not found");
    println!("Special: {}", text(special)?);

    Ok(())
}
