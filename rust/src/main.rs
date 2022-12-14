//              _       __   __ 
//     //\     | |      \ \ / / 
//    //_\\    | |       \ / /  
//   //___\\   | |___    / / \  
//  //     \\  |_____|  /_/ \_\ 

// Crates
use std::io;


fn main() {
// Outline of the project check the respective function for more info
    print_alx();
    password_check();
    
}

// Functions

fn print_alx(){
    //prints out my logo
    println!(r"              _       __   __ ");
    println!(r"     //\     | |      \ \ / / ");
    println!(r"    //_\\    | |       \ / /  ");
    println!(r"   //___\\   | |___    / / \  ");
    println!(r"  //     \\  |_____|  /_/ \_\ ");
    println!(r"                              ");
}
fn password_check(){
    //checks password
    let mut line = String::new();
        println!("PASSWORD:");
        std::io::stdin().read_line(&mut line ).unwrap();

        if line == "Joshua"{
            println!("HELLO PROFESSOR");
        }
        if line != "Joshua" {
            println!("PASSWORD: DENIED");
        }
}
