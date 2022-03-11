/* 
    Primitive datatypes:
    INTEGERS :
        unsigned : u8, u16, u32, u64, u128
        signed : i8, i16, i32, i64, i128
    FLOATS : f32, f64
    BOOLEAN : bool
    CHARACTERS : char
    TUPLES : (TYPE1, TYPE2, TYPE3)
    ARRAYS : fixed length [type:TYPE]
    VECTORS : mutable length arrays
*/


pub fn run(){
    /* default is i32 */
    let x = 2;

    /* default is f64 */
    let y = 2.3;

    /* explicit type */
    let z: i64 = 3333333333;

    /* find the max size of the variable type */
    println!(
        "x is {}, y is {} z is {}",
        x, y, z
    );
    println!("Max of z (i64) is : {}", std::i64::MAX);

    /* Boolean */
    let is_active: bool = true;
    println!("Is active : {}", is_active);

    /* boolean from expression */
    let is_greater: bool = 10.2 > 10.1;
    println!("10.2 is greater than 10.1 : {}", is_greater);

    /* chars unicode*/
    let a: char = 'a';
    let smiley_face : char = '\u{1f600}';
    println!("{:?}", (a, smiley_face)); 

}