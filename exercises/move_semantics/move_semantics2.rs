// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec = Vec::new();

    fill_vec(&mut vec);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
