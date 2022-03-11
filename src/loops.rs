/*
    iterate until a condition is met
*/
pub fn run(){
    let mut count: u8 = 0;

    // infinite loop
    loop{
        count += 1;
        println!("Number: {}", count);

        if count == 20{ // if count reaches 20, leave loop
            break;
        }
    }
    count = 0;
    // while loop
    // fizzbuzz challenge
    while count <= 100{
        println!("Number : {}", count);
        if count % 15 == 0{ // if its divisible by 3 and 5
            println!("FizzBuzz");
        } else if count % 3 == 0 { // if its divisible by 3
            println!("Fizz");
        } else if count % 2 == 0 { // if its divisible by 2
            println!("Buzz");
        }
        //increment counter
        count += 1;
    }

    // for loop
    // fizzbuzz challenge
    for count in 0..100{
        println!("Number : {}", count);
        if count % 15 == 0{ // if its divisible by 3 and 5
            println!("FizzBuzz");
        } else if count % 3 == 0 { // if its divisible by 3
            println!("Fizz");
        } else if count % 2 == 0 { // if its divisible by 2
            println!("Buzz");
        }
        //increment counter -> automatic!
        //count += 1;
    }

    
}