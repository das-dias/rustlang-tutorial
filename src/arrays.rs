/*
    Arrays : fixed length list of elements that are 
                of the same datatype
*/

// use namespace std-lib::mem
use std::mem;

pub fn run(){
    // new array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single array value
    println!("First value : {}", numbers[0]);

    // change the array
    let mut arr: [i32; 4] = [1, 2, 3, 4 ];
    arr[3] = 20;
    println!("Changed value : {}", arr[3]);

    // get the array length
    println!("array length : {}", arr.len());

    // arrays are stack allocated 
    println!("array occupies {} bytes", mem::size_of_val(&arr));

    // get a slice of an array
    let mut slice : &[i32] = &numbers;
    println!("Slice : {:?}", slice);

    slice = &numbers[2..3];
    println!("Slice : {:?}", slice);
}