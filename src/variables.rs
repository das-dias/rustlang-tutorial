/* 
    variables are imutable by default
    for declaring mutable variables, the keyword "mut" msut be present

    IT IS NOT REQUIRED TO DECLARE THE TYPE OF THE VAR EVERYTIME
*/


pub fn run(){
    /* imutable vars */
    let name = "Diogo";
    let age = 23; 

    /* impossible ! error will be generated */
    //age = 22;

    /* mutable age */
    let mut age_ = 22;
    age_ = 23;
    println!("My name is {} and I'm {} yrs old", name, age_);

    /* define constants */

     /* constants need to 
        be declared with a 
        datatype (i32 = signed integer 32 bits) 
     */
    const WILL: i32 = 9000;
    println!("My will is over {} !!", WILL);

    /* assign mutltiple (imutable) variables  */
    let (my_name, my_age) = ("Diogo", 23);

    println!(
        "{} is {}!",
        my_name,
        my_age
    );
    /* not possible */
    //my_age = 23;
}