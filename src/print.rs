pub fn run(){
    /* print a string */
    println!("Hello, HELL GRAVE!");

    /* print variables */
    println!("Number : {}", 1);

    /* print multiple variables */
    println!("My name is {} and I am {} years old, and I come from {}", "Diogo", 23, "Hell Grave");

    /* positional arguments */
    println!(
        "{0} is from {1} and she likes {2}",
        "Chica", "Azeitao", "to sleep"
    );

    /* named arguments - dictionary indexing */
    println!(
        "{name} is a cat and she's {adjective}",
        name = "Catila", 
        adjective = "Fucking Agressive"
    );

    /* placeholder traits */
    println!("Binary : {:b}, Hex : {:x}, Octal : {:o}", 10, 10, 10);

    /* special placeholder for debugging trait - print any type */
    println!("What's this : {:?}", ("this","is", "a tuple", 50));
    println!("What's a simpler this : {:?}", true);
}