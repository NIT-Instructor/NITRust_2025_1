fn play_with_pointers()
{
    println!("Hello from NIT Team!");
    let mut line: (/*p1*/(i32, i32),
                   /*p2*/(i32, i32)) = ((0, 0), (1, 1));

    let ref_poit1 = &line.0;
    //*ref_poit1 = (77, 77);

    println!("line.0 = {:?}", line.0);

    let ref_poit2 = &mut line.1;
    //*ref_poit3 = (99, 99);
    *ref_poit2 = (99, 99);

    let mut ref_poit = &mut line.0;
    ref_poit = &mut line.1;
    ref_poit = &mut line.0;
}

fn find_min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];

    for &num in numbers.iter().skip(1) {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    (min, max)
}



fn print_star_of_asterisks() {
    let star = "*";
    let mut stars = String::new();
    for _ in 0..10 {
        stars.push_str(star);
        println!("{}", stars);
    }
}

fn data_types() {
    // Numeric types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let unsigned: u8 = 255;
    
    // Boolean
    let is_active: bool = true;
    
    // Character
    let letter: char = 'A';
    
    // String types
    let string_literal: &str = "Hello";  // string slice
    let owned_string: String = String::from("World");
    
    // Tuple
    let tuple: (i32, f64, char) = (1, 2.5, 'X');
    
    // Array - fixed size
    let array: [i32; 4] = [1, 2, 3, 4];
    
    // Vector - growable array
    let mut vector: Vec<i32> = vec![1, 2, 3];
    vector.push(4);  // vectors can grow
    
    // Option type - handles nullable values
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    
    // Print everything
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Unsigned: {}", unsigned);
    println!("Boolean: {}", is_active);
    println!("Character: {}", letter);
    println!("String literal: {}", string_literal);
    println!("Owned string: {}", owned_string);
    println!("Tuple: {:?}", tuple);  // Debug print for tuple
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
    println!("Option Some: {:?}", some_value);
    println!("Option None: {:?}", no_value);
}

fn data_types2() {
    let inferred_int = 42;  // Rust infers i32
    println!("Inferred int: {}", inferred_int);
    
    // These will NOT work:
    //inferred_int = 42.5;    // Can't assign float to i32
    //inferred_int = "42";    // Can't assign string to i32
    
    // This won't work because variables are immutable by default
    //inferred_int = 43;      // Error: cannot assign twice to immutable variable
    
    // To allow changes, we need 'mut':
    let mut mutable_int = 42;
    println!("Mutable int: {}", mutable_int);
    // check if mutable_int is even and if so increment by 1
    if mutable_int % 2 == 0 {
        mutable_int += 1;
    }
    else {
        mutable_int += 2;
    }
    
    println!("Mutable int: {}", mutable_int);
    
    // Even with mut, we can't change type:
    //mutable_int = 42.5;     // Error: mismatched types
}

fn main() {
    let numbers = [4, 2, 8, 1, 9, 3, 5];
    let (min, max) = find_min_max(&numbers);
    println!("Min: {}, Max: {}", min, max);
    data_types();
    data_types2();
    print_star_of_asterisks();
    play_with_pointers();
}
