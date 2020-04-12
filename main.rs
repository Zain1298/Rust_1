
 const PI : f64 = 3.14159265;
 use std::io;
 fn main() {
    
     let mut input = String::new();
     io::stdin().read_line(&mut input);
     let input : f64 = input.trim().parse().unwrap();
    
     println!("The radius of circle is : {}",input);
     println!("The area of circle is : {}",PI*input*input);
 }