
use std::collections::{btree_map::Values, HashMap};



fn main() {

    let mut students : HashMap<String, u32> = HashMap::new();

    students.insert(String::from("Harsh"), 10);
    students.insert(String::from("Kush"), 30);
    students.insert(String::from("Akash"), 15);

    students.insert(String::from("Akash"), 55);   // OVER-RIDE FIELD

    students.entry(String::from("Puru")).or_insert(19);   // THIS CHECKS DATA, IF NOT PRESENT , THEN THIS MAKES ENTRY IN MAPS

    println!("{:#?}", students);

    // GETING STUDENT
    let student = students.get(&String::from("Harsh")).unwrap_or(&0);
    println!("Student Roll No : {student}");

    // ITERATING ON HASHMAP
    for (key, value) in &students {
        println!("{} -> {}", key , value)
    }
    
}
