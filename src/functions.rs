// strings as inputs (through reference)
fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}

// returning values (-> Return type)
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // do not add semicolon to return value
}

// parsing functions as inputs to other functions V1
// fn -> C-style function pointer expected
// !! Does not accept closure expressions ! Error
fn calculate1(n1: i32, n2: i32, opr:  fn(i32, i32)->i32) -> i32 {
    opr(n1, n2)
}

// parsing functions as inputs to other functions V2
// &dyn - dynamic dispatch of function pointer 
// Fn -> the parsed inputs are non mutable
// FnMut -> the parsed inputs are mutable
// FnOnce-> the function can be called once, after which it is consumed and never used again
// ! Accepts closure expressions
fn calculate2(n1: i32, n2: i32, opr: &dyn Fn(i32, i32)->i32) -> i32 {
    opr(n1, n2)
}

// parsing functions as inputs to other functions V3
// impl -> static dispatch of the function pointer
// a copy of the function pointer is created and afterwards destroyed
// ! Accepts closure expressions
fn calculate3(n1: i32, n2: i32, opr: impl Fn(i32, i32)->i32) -> i32 {
    opr(n1, n2)
}

pub fn run(){
    greeting("Ola", "Chica");
    let sum = add(4, 3);
    println!("Sum : {}", sum);

    // inline function / lambda declarations / closures

    // this way we can use outside variables
    let offset = 20;
    let add_nums = |n1: i32, n2: i32| n1+n2 + offset;
    let new_sum = add_nums(2,1);
    println!("New sum: {}", new_sum);

    // parsing functions as input to other functions
    println!("Result of 32 + 3 = {}", calculate1(32, 3, add)); // C style function pointer
    println!("Result of 32 + 3 = {}", calculate2(32, 3, &add)); // Rust style dynamic dispatch function pointer
    println!("Result of 32 + 3 = {}", calculate3(32, 3, add)); // Rust style static dispatch of function pointer

    // for more information on parsing functions as inputs : 
    //https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter
}

// * functions CAN BE declared below !! 