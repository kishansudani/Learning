use std::collections::HashMap;

fn median(array: &[i32]) -> i32 {
    let length = array.len();
    if length % 2 == 0 {
        let middle_right = length / 2;
        let middle_left = middle_right - 1;
        (array[middle_left] + array[middle_right]) / 2
    } else {
        array[length / 2]
    }
}

fn modes(array: &[i32]) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for value in array {
        let count = map.entry(*value).or_insert(0);
        *count += 1;
    }
    map
}

fn main() {
    let mut array_of_ints = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    array_of_ints.sort();
    let value = median(&array_of_ints);
    println!("{}", value);
    array_of_ints.push(2);
    array_of_ints.push(7);
    array_of_ints.push(4);
    array_of_ints.push(7);
    let value = median(&array_of_ints);
    println!("{}", value);
}
