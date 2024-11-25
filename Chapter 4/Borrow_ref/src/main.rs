


fn burrow_ref_imutable(s2: &String) ->  usize{
    s2.len() // Can read `s` but not modify it
}

fn burrow_ref_mutable(s2: &mut String) ->  usize{
    s2.push_str(",world");
    s2.len() // Can read `s` but not modify it
}

fn main() {

    //  THIS BORROWING IN IMMUTABLE

    let s1 = String::from("Hello");
    let len = burrow_ref_imutable(&s1);  // Borrow `s` immutably
    println!("String is : {s1} and length: {len}"); // `s` is still valid


    // THIS BORROWING IS MUTABLE

    let mut s2 = String::from("Hello");
    let len2 = burrow_ref_mutable(&mut s2);  // Borrow `s2` mutably
    println!("String is : {s2} and length: {len}"); // `s2` is still valid


}
