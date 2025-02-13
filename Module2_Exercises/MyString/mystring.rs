#[derive(Debug)]
struct MyString {
    data: Vec<char>,
}

// Standalone functions for MyString operations
fn new() -> MyString {
    MyString { data: Vec::new() }
}

fn push(s: &mut MyString, ch: char) {
    s.data.push(ch);
}

fn len(s: &MyString) -> usize {
    s.data.len()
}


fn main() {
    // Create a new MyString instance using the 'new' function
    let mut my_string = new();

    // Push characters onto the string
    push(&mut my_string, 'H');
    push(&mut my_string, 'e');
    push(&mut my_string, 'l');
    push(&mut my_string, 'l');
    push(&mut my_string, 'o');

    // Print the MyString (using Debug trait implementation)
    println!("My string: {:?}", my_string);

    // Get and print the length of the string
    let length = len(&my_string);
    println!("Length of my string: {}", length);

    let my_string2 = new(); // Create a new empty MyString
    let length2 = len(&my_string2); // Borrow my_string2 (immutable reference) to get its length

    println!("Length of my_string2: {}", length2);


    let mut my_string3 = new();
    push(&mut my_string3, 'W');
    push(&mut my_string3, 'o');
    push(&mut my_string3, 'r');
    push(&mut my_string3, 'l');
    push(&mut my_string3, 'd');

    println!("My string 3: {:?}", my_string3);

    // my_string3 is moved here 
    print_string(my_string3); 
    
    // println!("My string 3 after move: {:?}", my_string3); // This would be an error, as my_string3 moved to function input variable

    let mut my_string4 = new();
    push(&mut my_string4, '!');
    push(&mut my_string4, '!');

    print_string_ref(&my_string4); // Borrowing (immutable refernce) example

    println!("My string 4 after borrow: {:?}", my_string4); // my_string4 still owned here

}

// Function that takes ownership of MyString
fn print_string(str_to_print: MyString) {
    println!("String inside print_string: {:?}", str_to_print);
    // s is dropped here when the function ends
}

// Function that borrows a MyString
fn print_string_ref(str_to_print: &MyString) {
    println!("String inside print_string_ref: {:?}", str_to_print);
    // s is not dropped here
}