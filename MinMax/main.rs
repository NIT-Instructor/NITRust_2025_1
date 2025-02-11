
fn find_min_max(numbers: &[i32]) -> (i32,i32) {
    if numbers.is_empty() {
        return (0, 0);
    }

    let mut min = numbers[0];
    let mut max = numbers[0];


    for &num in numbers {  
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


     // Example with an array
     let my_array = [10, 4, 7, 1, 8,9,15];
     let array_slice: &[i32] = &my_array;
     let (min_array, max_array) = find_min_max(array_slice);
      println!("Minimum: {}, Maximum: {}", min_array, max_array);
}