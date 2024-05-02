fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;

    let mut vecs = vec![2, 4, 6, 8, 10];
    println!("{:?}", vecs);
    vecs.pop();
    vecs.push(12);

    println!("{}", concat_string(String::from("Hello")));
}

fn concat_string(word: String) -> String {
    word + " World"
}

fn control_flow(value: i64) {
    if value == 1 {
        println!("The value is one");
    } else if value < 25 {
        println!("The value is less than 25");
    } else if value >= 25 && value < 50 {
        println!("The value is greater than 25 but less than 50");
    } else {
        println!("The value is greater than 50");
    }
}
