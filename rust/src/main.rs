

fn main() {
// Outline of the project

    println!(r"              _       __   __ ");
    println!(r"     //\     | |      \ \ / / ");
    println!(r"    //_\\    | |       \ / /  ");
    println!(r"   //___\\   | |___    / / \  ");
    println!(r"  //     \\  |_____|  /_/ \_\ ");

    password_check();    

}

// My comands

fn password_check(){
    
    let mut line = String::new();
        println!("PASSWORD:");
        std::io::stdin().read_line(&mut line ).unwrap();

        if line == "Joshua"{
            println!("HELLO PROFESSOR");

        }
        else {
            println!("PASSWORD: DENIED");
        
        }
}


