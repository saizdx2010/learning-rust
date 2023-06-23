fn main() {
    say_my_name("Sai Prashanth");
    sum_this_up(1, 2);
    return_this_num(10);
    simple_if();
    is_number_divisable_by_3();
    if_condition_in_let_statement();
    // rust_loop();
    rust_loop_part_2();
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

fn simple_if() {
    let number = 4;

    if number != 4 {
        println!("Number is not four");
    }
    if number == 4 {
        println!("Number is four");
    }
    if number < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is more than 10");
    }
}

fn is_number_divisable_by_3() {
    let number = 4;

    if number % 3 == 0 {
        println!("Number is divisable by 3");
    } else {
        println!("Number is not divisable by 3");
    }
}

fn if_condition_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The condition is {condition} so the number is {number}");
}

fn rust_loop() {
    let my_name = "Sai Prashanth";

    loop {
        println!("{my_name}");
    }
}

fn rust_loop_part_2() {
    let mut count = 0;

    loop {
        count += 1;

        println!("Counter value is {}", count);

        if count == 10 {
            break println!("Counter value has reached {}", count);
        }
    }
}

fn use_of_loop_labels() {
    'outer: loop {
        println!("This is the outer loop");

        'inner: loop {
            println!("This is the inner loop");

            break 'outer;
        }

        println!("This outer point will never be reached by the program");
    }

    println!("Exited the outer loop");
}
