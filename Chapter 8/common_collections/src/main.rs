

#[derive(Debug)]
enum SpreadSheet {
    Age(u32),
    Name(String),
    Active(bool),
}






fn main() {


    
    let mut vec = Vec::new();   // VECTOR CREATION  , AND THIS IS MUTABLE
    let  vec2 = vec![5,6,7];    // WE WILL CREATE VECTOR WITH INITIAL VALUES, FOR MAKE IT IMMUTABLE

    // SAME TYPE OF ELEMENTS ARE INSERT IN VECTOR
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);



    println!("VEC : {:?}", vec);
    println!("VEC2 : {:?}", vec2);

    // FOR ACCCESS VECTOR WITH INDEX
    println!("VALUE AT 2 INDEX  : {:?}", vec[2]);


    // println!("VALUE AT 30 INDEX  : {:?}", vec[30]);   THIS CRASH THE PROGRAM

    // SO , IF WE ACCESS 30 INDEX,THAT NOT PRESENT IN OUR VECTOR, THEN OUR PROGRAM CRASH , SO WE USE THIS TO PREVENT CRASH 

    let thirty_index = vec.get(90).unwrap_or(&-1);  

    println!("VALUE AT 30TH INDEX : {:?}", thirty_index);


    // FOR ITERATING IN VECTOR

    for i in &vec{
        println!("Iterate On Vector : {:?}", i);
    }


    // WE CREATE ENUM  VECTOR  WITH THAT
    let vec3 :Vec<SpreadSheet> = vec! [
        SpreadSheet::Age(30),
        SpreadSheet::Name(String::from("Harsh")),
        SpreadSheet::Active(false),
    ];

    println!("{:#?}", vec3);




    



}
