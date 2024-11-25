

fn takes_ownerShip(s2: String) {
            println!("{s2}");   // `s2` owns the heap memory
}  // `s2` is dropped here

fn ownerShip_takes_and_give_back(s4:String) -> String {
    s4  // Return ownership
}




fn main() {

        let s1 = String::from("Hello");
        // println!("{s1}");
        takes_ownerShip(s1); // Ownership moved; `s` is no longer valid


        let s2 = String::from("world;");
        let s3 = ownerShip_takes_and_give_back(s2); // Ownership moves to `s3`
        println!("{s3}");  // Valid because `s3` now owns the memory

}
