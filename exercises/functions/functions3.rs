// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(Some(5));  // you can just simply pass a u32 here for argument
}

// but here I'm showing how can you pass an optional argument
// I think, rust doesn't have support for variadic parameters in general
fn call_me(num: Option<u32>) {
    for i in 0..num.unwrap() {
        println!("Ring! Call number {}", i + 1);
    }
}
