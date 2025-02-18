fn find_min_max(numbers: &[i32]) -> (i32, i32) {
    if numbers.is_empty() {
        panic!("List is empty");
    }

    let mut min = numbers[0];
    let mut max = numbers[0];

    for &num in numbers.iter() {
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
    let numbers1 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let numbers2 = [-10, 0, 10, 20, -20, 30];
    let numbers3 = [42];

    let (min1, max1) = find_min_max(&numbers1);
    println!("For numbers1: Min = {}, Max = {}", min1, max1);

    let (min2, max2) = find_min_max(&numbers2);
    println!("For numbers2: Min = {}, Max = {}", min2, max2);

    let (min3, max3) = find_min_max(&numbers3);
    println!("For numbers3: Min = {}, Max = {}", min3, max3);
}
