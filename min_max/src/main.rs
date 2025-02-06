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

fn main() {
    let numbers = [4, 2, 8, 1, 9, 3, 5];
    let (min, max) = find_min_max(&numbers);
    println!("Min: {}, Max: {}", min, max);
}
