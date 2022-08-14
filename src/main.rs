// A temperature converter
use std::io;
//use std::cmp;
fn main(){
    println!("Type FC to convert from fahrenheit to celsius and CF to do the opposite");
    let mut choice : String = String::new();
    io::stdin().read_line(&mut choice).expect("Could'nt read line");
    let mut fahrenreit: String = String::new();
    let mut celsius: String = String::new();
    if choice.trim() == "FC"{
        println!("Enter temperature in fahrenreit:");
        
        io::stdin()
        .read_line(&mut fahrenreit)
        .expect("Could'nt read line");
        let fahrenreit: f32 = fahrenreit
        .trim()
        .parse()
        .expect("Expects a numeric string");
        let celsius: f32 = {((fahrenreit - 32.0 ) * 5.0) / 9.0};
        println!("{} ºC",celsius);
    }else if choice.trim() == "CF"{
        println!("Enter temperature in celsius");
        io::stdin()
        .read_line(&mut celsius)
        .expect("Could'nt read line");
        let celsius: f32 = celsius
        .trim()
        .parse()
        .expect("Expects a numeric string");
        let fahrenheit: f32 = {((celsius * 9.0 ) / 5.0) + 32.0};
        println!("{}ºF",fahrenheit );
    }else{
        println!("Invalid temperature unit");
    }
    //println!("Enter temperature in fahrenreit:");
    
    
    
    
    
    
    
    
    
    
}
