/*
    Primitive strings:
        immutable fixed length string in static memmory
    
    String : mutable, heap-allocated data structure in dynamic memory
*/
pub fn run(){
    /* imutable */
    let hello1 = "hello";

    println!("{} is of length {}", hello1, hello1.len());

    /* mutable string (has to have mut) */
    let mut hello2 = String::from("Hoy");
    println!("{} is of length {}", hello2, hello2.len());

    /* push / add a char */
    hello2.push('!');

    /* push / add string */
    hello2.push_str(" tudo bom?");

    println!("{} is of length {}", hello2, hello2.len());

    /* capacity in bytes */

    // not possible for imutable strings
    //println!("Capacity of hello1 : {}", hello1.capacity());
    println!("Capacity of hello2 : {}", hello2.capacity());

    // check if mutable str is empty
    println!("Is empty hello2 empty ? {}", hello2.is_empty());

    // check if contains a substring
    println!("Hello2 contains Hoy : {}", hello2.contains("Hoy"));

    // replace substring
    println!("Replace : {}", hello2.replace("Hoy", "Hello There"));

    // loop through string by white spaces
    for word in hello2.split(' '){
        println!("{}",word);
    }
    // second way
    for word in hello2.split_whitespace(){
        println!("{}",word);
    }

    // create string with a given capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('c');
    println!(
        "s : {} capacity: {}, length: {}",
        s, s.capacity(), s.len()
    );  

    // asserting test

    // check is length of s is 3; if not, throw exception (panick)
    assert_eq!(2, s.len()); // passes, nothing happens
    
    assert_eq!(3, s.len()); // panicks
}