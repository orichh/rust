fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone() + "test";
    let s1_length = get_string_length(&s1);

    println!("s1 = {}, s2 = {}", s1, s2);
    writes_input(&s1);
    writes_input(&s1_length.to_string());

    add_string_to_string(&mut s2, "test");
    println!("s2 after adding: {}", s2);

    add_string_to_string_multiple(&mut s2, "added", 10);
    println!("s2 after adding: {}", s2);

    for _ in 0..100_000 {
        let target = 150;
        let fibonacci_result = find_fibonacci(target);
        println!("fibonacci at {}: {}", target, fibonacci_result);
    }

    create_slice("hello world");
}

fn writes_input(s: &String) {
    println!("{}", s);
}

fn get_string_length(s: &String) -> usize {
    s.len()
}

fn add_string_to_string(input: &mut String, add_string: &str) {
    input.push_str(&add_string)
}

fn add_string_to_string_multiple(input: &mut String, add_string: &str, num_times: u8) {
    for _ in 0..num_times - 1 {
        input.push_str(&add_string)
    }
}

fn find_fibonacci(target: u32) -> u128 {
    fibonacci(0, 1, target, 1)
}

fn fibonacci(first_num: u128, second_num: u128, target: u32, current: u32) -> u128 {
    println!("{first_num}");
    if target != current {
        fibonacci(second_num, first_num + second_num, target, current + 1)
    } else {
        second_num
    }
}

fn create_slice(s: &str) {
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}
