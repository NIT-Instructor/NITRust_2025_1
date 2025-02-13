struct MyString {
    chars: Vec<char>,
}

fn new(chars: Vec<char>) -> MyString {
    MyString { chars }
}

fn push(str: &mut MyString, c: char) {
    str.chars.push(c);
}

fn len(str: &MyString) -> usize {
    str.chars.len()
}

impl std::fmt::Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.chars {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[test]
fn custom_string_test() {
    let chars = vec!['g', 'r', 'o', 'u', 'p'];
    let mut my_string = new(chars);
    assert!(len(&my_string) == 5);

    push(&mut my_string, '2');
    assert!(len(&my_string) == 6);

    println!("{my_string}");
}
