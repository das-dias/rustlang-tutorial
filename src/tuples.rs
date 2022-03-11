/*
    Tuples group values, and these values
    can be of different types
*/

pub fn run(){
    let person: (&str, &str, i8) = ("Diogo", "Barreiro", 23);

    println!(
        "{} is from {} and is {}", 
        person.0, person.1, person.2
    );
}