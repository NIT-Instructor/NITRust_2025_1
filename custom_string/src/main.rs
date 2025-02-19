#[derive(Debug)]
struct MyString {
    data: Vec<char>,
}

// Free functions for MyString operations
fn new() -> MyString {
    MyString {
        data: Vec::new(),
    }
}

fn push(string: &mut MyString, ch: char) {
    string.data.push(ch);
}

fn len(string: &MyString) -> usize {
    string.data.len()
}

// Additional utility function to help with testing
fn from_str(s: &str) -> MyString {
    MyString {
        data: s.chars().collect(),
    }
}

fn to_string(string: &MyString) -> String {
    string.data.iter().collect()
}

fn main() {
    // Test creating an empty string
    println!("Testing MyString implementation");
    println!("==============================\n");
    
    let mut s1 = new();
    println!("Created empty string: {:?}", s1);
    println!("Length: {}", len(&s1));
    
    // Test pushing characters
    println!("\nTesting push operation:");
    push(&mut s1, 'H');
    push(&mut s1, 'e');
    push(&mut s1, 'l');
    push(&mut s1, 'l');
    push(&mut s1, 'o');
    
    println!("After pushing 'Hello': {:?}", s1);
    println!("Length: {}", len(&s1));
    println!("As string: {}", to_string(&s1));
    
    // Test creating from existing string
    println!("\nTesting string creation from str:");
    let s2 = from_str("World!");
    println!("Created from 'World!': {:?}", s2);
    println!("Length: {}", len(&s2));
    println!("As string: {}", to_string(&s2));
    
    // Test empty string behavior
    println!("\nTesting empty string behavior:");
    let empty = new();
    println!("Empty string length: {}", len(&empty));
    println!("Empty string debug: {:?}", empty);
    println!("Empty string as string: '{}'", to_string(&empty));
}
