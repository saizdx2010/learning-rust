fn main() {
    let mut x = 5;
    println!("The number is {}", x);

    x = 6;

    println!("The number is {}", x);

    const THIS_NUMBER_IS_CONSTANT: i32 = 1;

    println!("print the constant value {}", THIS_NUMBER_IS_CONSTANT);

    let name: &str = "Sai Prashanth";

    {
        let name: &str = "Prashanth Sai";

        println!("The name is {}", name);
    }

    println!("Really, the name is {}", name);

    let spaces: &str = "    ";

    {
        let spaces: usize = spaces.len();
        println!("Spaces length : {}", spaces);
    }

    println!("spaces is : {}x", spaces);

    let numbers_addition = 5 + 1;

    let number_subtraction = 6 - 12;

    let number_multiplication = 10 *9;

    let number_division = 10 / 5;

    let number_remainder = 10 % 5;
}
