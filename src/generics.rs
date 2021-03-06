mod zesp; 
use zesp::Zespolona;

fn bridge<G>(v: &G) -> &G {
    v
}

fn main() {
    let y = 6;
    let x = 3.2;
    let a = "Ciocia";
    let d = 'z';
    let number = Zespolona::new(5 as f64, 3 as f64); 
    println!("{}", bridge(&y));
    println!("{}", bridge(&x));
    println!("{}", bridge(&a));
    println!("{}", bridge(&d));
    println!("{}", bridge(&number));
}