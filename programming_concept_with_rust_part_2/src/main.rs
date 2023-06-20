fn main() {
    // say_my_name("Sai Prashanth");
    sum_this_up(1, 2);
    return_this_num(10);
}

fn say_my_name(name: &str) {
    println!("My name is {}", name);
}

fn sum_this_up(num1: i16, num2: i16) {
    let total = num1 + num2;
    println!("{num1} + {num2} = {total}")
} 

fn return_this_num(num: i16) -> i16 {
    return num;
}
