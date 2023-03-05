use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();

    for i in 0..100 {
        print!("|");
        pbarprint(i);
        print!("|");
        print!(" Processing {}%", i);
        print!("\r");
        // or
        // stdout.write(format!("\rProcessing {}%...", i).as_bytes()).unwrap();

        stdout.flush().unwrap();
        sleep(Duration::from_millis(20));
    }
    println!();
}
fn pbarprint(i:i32){
    for k in 1..i{
        // print!("\x1b[41m|\x1b[0m")
        // print!("\x1b[41m \x1b[0m")   
        // print!("░")
        print!("━")
        // print!("#")
    }
    for j in 1..100-i{
        // print!(" ")
        print!(" ")
    }
}
