mod calculator;
//mod concatenate;



fn main() {
    // let string1 = String::from("Hello, ");
    // let string2 = String::from("World!");
    // let concatenated_string = concatenate_strings(&string1, &string2);
    // println!("{}", concatenated_string);
    let addition = calculator::Calculator::Addition { x: 3, y: 2 };
    let subtraction = calculator::Calculator::Subtraction{ x: 1, y: 1 };
    let multiplication = calculator::Calculator::Multiplication{ x: 3, y: 1 };
    let division = calculator::Calculator::Division{ x: 9, y: 3 };
    calculator::calculate(addition);
    calculator::calculate(subtraction);
    calculator::calculate(multiplication);
    calculator::calculate(division);
}

