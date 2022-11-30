//import crates
use std::fs;
use std::io;
use std::io::Write;
fn main() {
    let mut input: String = String::new(); /* input */
    println!("Spam?");
    io::stdin().read_line(&mut input).expect("No spam");
    input = input.trim().to_lowercase();
    //work out whether to spam or not
    if input == "spam" {
        hello_world();
    } else {
        println!("No spam for you :(")
    }
}
//spam
fn hello_world() {
    let mut file = fs::File::create("spam.txt").expect("create failed"); /* make file */
    let mut i = 0; /* make i */
    
    
    
    let mut spamoutput: String = String::new(); /* spamout text */
    println!(r"Text to spam?");
    io::stdin().read_line(&mut (spamoutput)).expect("bad text"); /* make lines */

    
    
    
    loop {
        file.write_all(spamoutput.as_bytes())
            .expect("Create failed"); /* write to file */
        i += 1;
        if i == 5000 {
            break;
        }
    }
}
