mod generated;

fn main() {
    println!("Hello, world!");
    println!("==========================");
    println!("");

    let file = "assets/test.wav";
    unsafe {
        generated::avFormatOpenInput(file.as_ptr() , file.len());
    };

    println!("");
    println!("These are the essential meta data of the file. Much more to come :)");
}
