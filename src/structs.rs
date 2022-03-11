/**
 * Structs are used to create custom data types
 */

 // traditional C style struct
struct RGBColor{
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct 
struct BGRColor(u8, u8, u8);

// Classes - Object Oriented Structs
struct Person{
    first_name: String,
    last_name: String,
    age: u8
}

//create Class's methods
impl Person{
    
    //Constructor
    fn new(_first_name: &str, _last_name: &str, _age: u8) -> Person 
    {
        Person{
            first_name: _first_name.to_string(),
            last_name: _last_name.to_string(),
            age: _age
        }
    }

    // class associated methods

    // return full name
    fn get_fullName(&self) -> String {
        // format macro works like print! macro but doesn't print to console
        format!("{} {}", self.first_name, self.last_name) 
    }
    // set last name
    fn set_lastName(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    // get full name using a tuple as output
    // way one : using self directly, transferring ownership of self object,
    //             not preventing self from being consumed and destructed 
    //              after calling this method
    fn name_toTuple1(self) -> (String, String){
        (self.first_name, self.last_name)
    } // when directly returning class attributes (variables),
        // don't parse self as reference!


    // way two : using a borrow/copy of self,
    //              preventing self from being consumed and destructed 
    //              and allowing this method to be called multiple times
    fn name_toTuple2(&self) -> (&String, &String){
        (&self.first_name, &self.last_name)
    }
    fn name_fromTuple(&mut self, names: (&str, &str)){
        self.first_name = names.0.to_string();
        self.last_name = names.1.to_string();
    }
}

pub fn run(){

    // declare the traditional struct variable

    // way one
    let mut c = RGBColor{
        red: 255,
        green: 0,
        blue: 0
    };
    println!("My new color : Red: {}, Green: {}, Blue: {}", c.red, c.green, c.blue);
    // way two
    let mut c2: RGBColor = RGBColor{
        red:0,
        green:0,
        blue:0
    };
    c2.red = 255;
    c2.green = 255;
    c2.blue = 0;    
    println!("My new color : Red: {}, Green: {}, Blue: {}", c2.red, c2.green, c2.blue);

    // declaring tuple struct
    let mut c3 = BGRColor(255,255,0);
    c3.2 = 125;
    println!("Color BGR : Blue {}, Green {}, Red {}", c3.0, c3.1, c3.2);

    // create a new person
    let mut new_person = Person::new("Chica", "Catita", 23);
    println!(
        "Person {} {} is {} years old!", 
        new_person.first_name, 
        new_person.last_name,
        new_person.age
    );

    println!("Person {} is {} years old!", new_person.get_fullName(), new_person.age);

    new_person.set_lastName("Dias");

    println!("Person {} Married!", new_person.get_fullName());

    println!("Person {:?} is returned as a tuple", new_person.name_toTuple2() );

    new_person.name_fromTuple( ("Chiquita", "Bixinho") );

    println!("Person {:?} is again returned as a tuple", new_person.name_toTuple2() );

    //use ownership transferring method, moving self/new_person object from previous allocated memory
    println!("Person {:?} is again returned as a tuple and new_person is moved", new_person.name_toTuple1() );
    //println!("{}", new_person.first_name); // error -> not possible, because new_person ref pointer was moved

    // more on moved object seferences: 
    // https://stackoverflow.com/questions/28158738/cannot-move-out-of-borrowed-content-cannot-move-out-of-behind-a-shared-referen
}   
