use std::fmt::Debug;

#[derive(Debug)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    print!("Made {:?}", x);
    println!("Made a ToDrop!");
}
