#[allow(dead_code)]
fn min_max(slice: &[i32]) -> (i32, i32) {
    if slice.is_empty() {
        panic!("Slice is empty");
    }

    let mut max = slice[0];
    let mut min = slice[0];

    for &num in slice {
        if num > max {
            max = num;
        }

        if num < min {
            min = num;
        }
    }

    (min, max)
}

#[test]
fn min_max_test() {
    let list = [3, 4, 5, 1, 12, 112122, 111];

    let (min, max) = min_max(&list);

    println!("The min is {}, the max is {}", min, max);
}
