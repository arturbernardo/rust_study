fn main() {

    heap_memory();

    string_move();

    string_move_with_clone();

    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';
    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);

    let mut message = "Hello Rust";
    println!("Init: {}", message);
    message = "Hello There!";
    println!("Running: {}", message);
    let mesg = print_welcome("some message");
    println!("{}", mesg);
    some_types();

}

fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    let msg = "Hi There Again!";
    msg
    // return msg;
}

fn some_types() {

    // u32 -> 2**32 -1
    let num = -10;
    println!("{}", num);

    // u8 -> unisigned integer of bits
    //2**8 -1 = 255
    let small_num: u8 = 10;
    println!("{}", small_num);

    // -2**7 to 2**7 -1
    // -128 to 127
    let small_num_2: i8 = -10;
    println!("{}", small_num_2);

    // operating system related. u32 or u64 (i32 or i64)
    let sys_num: isize = -10;
    println!("{}", sys_num);

    let sys_num_2: usize = 10;
    println!("{}", sys_num_2);

    let float_num: f32 = 3.14;
    println!("{}", float_num);

    let float_num_2 = 3.23232423;
    println!("{}", float_num_2);

    let tup: (i32, &str, u8) = (20, "3.12", 1);
    println!("{}", tup.1);

    //Tupla
    let (a,b,c) = tup;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    //List
    let x = [1,5,6,7];

    println!("{}", x[2]);

    //"range"
    let y = [2; 6];

    println!("{}", y[5]);

}

fn heap_memory() {
    let message = "heap_memory oi";
    let message_2 = message;

    println!("{}", message);

    let message_str = String::from("heap_memory Oi");
    let message_str_2 = message_str;

    // nao existe mais em memória.
    // println!("{}", message_str);
    println!("{}", message_str_2);
}

fn string_move() {
    let message_str = String::from("string_move Oi");
    string_moved_here(message_str);

    // nao existe mais em memória.
    // println!("{}", message_str);
}

fn string_moved_here(a: String) {
    println!("string_moved_here: {}", a);
}


fn string_move_with_clone() {
    let message_str = String::from("Oi");
    string_moved_here(message_str.clone());

    println!("string_move_with_clone: {}", message_str);
}
