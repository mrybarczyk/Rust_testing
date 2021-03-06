mod zesp; 
use zesp::Zespolona;

fn main() {

    let number1 = Zespolona::new(5 as f64, 3 as f64); 
    let number2 = Zespolona::new(5 as f64, 3 as f64); 
    
    println!("{}", number1 + (3.2 as f64));
    println!("{}", (3.2 as f64) + number2);
    println!("{}", number1 * number2);
    println!("{}", number1 / (2 as f64));
    
}