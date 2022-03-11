pub fn run(){
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true; 
    //generic if else, else if

    // and : &&
    // or : || (C style)
    if (age >= 18 && check_id) || (knows_person_of_age && check_id) {
        println!("Bartender says : What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender says: Get the fuck out kid!");
    } else {
        println!("Bartender says: I need to see your ID");
    }

    // short hand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age: {}", is_of_age);
}