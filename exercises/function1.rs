/* Make this compile
fn main() {
    call_me(3);
}

fn call_me(num) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
} */

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
