fn main() {
    let v1 = vec![1, 2, 3]; // `v1` owns the vector
    let v2 = v1[0]; // Ownership is moved to `v2`
    println!("{:?}", v1[2]); // Error: `v1` is no longer valid
    println!("{:?}", v2); // Works: `v2` owns the vector
}