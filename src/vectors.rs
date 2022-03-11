/*
    Vectors: mutable length list of elements that are 
                of the same datatype
*/

// use namespace std-lib::mem
use std::mem;

pub fn run(){
    // new vector
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single vector value
    println!("First value : {}", numbers[0]);

    // change the vector
    let mut arr: Vec<i32> = vec![1, 2, 3, 4 ];
    arr[3] = 20;
    println!("Changed value : {}", arr[3]);

    // get the vector length
    println!("vector length : {}", arr.len());

    // vectors are heap allocated, but work like a stack
    println!("vector occupies {} bytes", mem::size_of_val(&arr));

    // get a slice of a vector
    let mut slice : &[i32] = &numbers;
    println!("Slice : {:?}", slice);

    slice = &numbers[2..3];
    println!("Slice : {:?}", slice);

    // push values into vectors
    println!("arr : {:?}", arr);
    arr.push(5);
    arr.push(6);
    println!("arr changed : {:?}", arr);

    // pop values from vectors
    arr.pop();
    println!("arr changed again : {:?}", arr);

    // loop through vectors
    //imutable iterator
    for val in numbers.iter(){
        println!("Number of numbers: {}", val);
    }

    // change the elements while looping
    // mutable iterator (only works with mutable vars)
    for val in arr.iter_mut(){
        *val = *val * 2;
        //*val *= 2;
        println!("changed arr val: {}", val);
    }
    println!("new arr : {:?}", arr);
    
}