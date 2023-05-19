
pub enum Calculator {
    Addition{ x: i32, y: i32 },
    Subtraction{ x: i32, y: i32 },
    Division{ x: i32, y: i32 },
    Multiplication{ x: i32, y: i32 },
}  
pub fn calculate(operation: Calculator)
{
    match operation 
    {
        Calculator::Addition{x,y} => 
        {
            let solution = x + y;
            println!("{} + {} = {}",x,y,solution);
        }
        Calculator::Subtraction { x, y } => 
        {
            let solution = x - y;
            println!("{} - {} = {}",x,y,solution);
        }
        Calculator::Division{x,y} => 
        {
            let solution = x / y;
            println!("{} / {} = {}",x,y,solution);
        }
        Calculator::Multiplication{x,y} => 
        {
            let solution = x * y;
            println!("{} * {} = {}",x,y,solution);
        }
    }
}






