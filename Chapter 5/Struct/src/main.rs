#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age:u32,
}

struct Rectangle {
    width:u32,
    height:u32,
}
struct Square {

}

// MUTABLE OWNERSHIP
// METHOD USING IMPL
impl Rectangle{
    fn area(self)->u32{  
        self.width * self.height
    }
}

// ASSOCIATED FUNCTIO USING IMPL
// CONSTRUCTOR
impl Square {
    fn area_of_four_sides(length:u32, breadth:u32)->u32{
        length*breadth
    }
}

fn main(){
   let user1= User{
        username:String::from("Harsh"),
        email:String::from("Harsh@gmail.com"),
        age:20,
    };
    let user2 = User{
        age:23,
        email:String::from("Akash@gmail.com"),
        ..user1  // ALL FIELDS ARE MOVED FROM USER 1 , OTHERS FROM GIVE IN USER2
    };

    println!("{} , {}", user1.age, user1.email);
    // println!("{}", user1.username);  WE CANT ACCESS THIS, BECAUSE THIS VALUE MOVES TO USER 2;
    println!("{}, {}, {}", user2.username, user2.email, user2.age);


    let rect1 = Rectangle{
        width:30,
        height:30,
    };

    println!("Area : {}", rect1.area());
    // println!("Area : {:?}", rect1); HERE WE DONT USER RECT, BECAUSE WE MOVES OWNERSHOP TO METHOD



    let sq1 = Square::area_of_four_sides(10,10);
    println!("Area of 4 Sides : {}", sq1);





}