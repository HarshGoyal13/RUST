
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    // Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them
    let mut hello = String::from("नमस्ते");  

    println!("{:?}", hello);


    // IN BYTES
    for i in hello.as_bytes(){
        println!("In Byptes : {i}");
    };

    // IN SCALAR
    for i in hello.chars(){
        println!("In Scalar : {i}");
    };


    // IN GRAPHEME
    for e in hello.graphemes(true).collect::<Vec<&str>>() {
        println!("In Grapheme : {e}");
    };
    
}
