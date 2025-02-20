#[derive(Debug)]
#[derive(Clone)]
struct  MyString{
    st:Vec<char>,
}

fn new() -> MyString{
    let my_str:MyString = MyString{st: Vec::new()};
    my_str
}

fn push(ms: &mut MyString, c:char) {
    ms.st.push(c);
}

fn len(ms: &mut MyString) -> usize {
    return ms.st.len();
}

fn main()
{
    let mut st:MyString=new();
    push(&mut st, 'a');
    push(&mut st, 'b');
    push(&mut st, 'c');
    //now pring lenght of string
    println!("length of string is: {}",len(&mut st));
    //now add some more chars
    push(&mut st, 'd');
    push(&mut st, 'e');
    push(&mut st, 'f');
    //print length of string again
    println!("length of string is: {}",len(&mut st));
}