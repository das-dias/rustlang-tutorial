/**
 * Point to a resource in memmory
 */

pub fn run(){
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values : {:?}", (arr1,arr2));

    // with non-primitive data types, assigning a variable to another var
    // needs to be done using reference pointers
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;   // vec1 and vec2 need to point to the same place
                        // in the heap they are declared in

    println!("Values 2 : {:?}", (&vec1, &vec2)); // can unpack vec2 from &vec2
    println!("Values 2 : {:?}", (&vec1, vec2)); // both ways work

    //(same thing happens with strings!)
}