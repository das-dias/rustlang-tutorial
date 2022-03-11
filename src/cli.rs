use std::env;

pub fn run(){
    // collect arguments from console
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    // clone allocates memory in the application to store the parsed strings
    let app_dir = argv[0].clone();
    let command = argv[1].clone();
    let mut rcvd_status: String= String::from("");
    if argc > 2{
        rcvd_status = argv[2].clone();
    }
    println!("App dir: {}", app_dir);
    if command == "help"{
        println!("Help was requested!");
    } else if command == "status" {
        
        println!("status is {}", rcvd_status);
    }
   
}