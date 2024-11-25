
#[derive(Debug)]
struct Point <T> {   // IN THIS WE ONLY GIVE SIMIMAR T VALUE
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2 <T, U> {   // IN THIS WE  GIVE DIFFERENT T VALUE
    x: T,
    y: U,
}




fn largest_num<T: std::cmp::PartialOrd>(list:&[T]) -> &T {

    let mut result = &list[0];

    for item in list{
        if item > result {
            result = item;
        }
    }

    result
}

fn main() {

    let _list = vec![2,3,466,7];
    let _list_2 = vec![2.1,3.1,4.66,7.9];
    let ans = largest_num(&_list_2);
    println!("Num : {ans}");
    

    let int_point = Point{x:6, y:7};
    let float_point = Point{x:6.5, y:7.8};
    let INT_FLOAT = Point2{x:6, y:7.8};

    println!("INT Point : {:?}", int_point);
    println!("FLOAT Point : {:?}", float_point);   
    println!("INT FLOAT : {:?}", INT_FLOAT);   

}
