fn main() {
    call_me(20);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring!, Call number {}", i + 1);
    }
}