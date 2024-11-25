




// IF I NOT USE , <'a> THEM IT'S SHOWS LIFETIME MISSING ERROR

fn valid_num<'a>(x:&'a i32, y: &'a i32) -> &'a i32{
    if x > y {
        return x;
    }
    else{
        y
    }
}


fn main() {
    let num = valid_num(&7, &5);
    println!("Result : {num}");
}


