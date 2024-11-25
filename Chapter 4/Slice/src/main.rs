



/*

A slice is a reference to a contiguous sequence of elements in a collection.
Unlike arrays, slices are dynamically sized and don’t own the data they reference.
Syntax: &[T] for an immutable slice and &mut [T] for a mutable slice.


*/

fn main() {


    // SLICE 


    // IN STRING
    let s1 = String::from("hello World");
    let slice = &s1[0..5];  
    println!("{:?}", slice);

    // IN ARRAY  || IMMUTABLE 
    let arr:[i32;5] = [1,2,3,4,5];
    let arrSlice = &arr[0..3];
    println!("{:?}", arrSlice);

    // MUTABLE
    let mut arr2:[i32;5] = [1,2,3,4,5];
    let immuatanleSlice = &mut arr2[0..2];
    immuatanleSlice[0] = 10;
    println!("{:?}", immuatanleSlice);



}
