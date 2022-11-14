// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a = [
        1, 2, 3, 4, 5, 65, 7, 8, 9, 9, 0, 0, 3, 4, 34, 234, 4, 3, 342, 234, 24, 24, 24, 242, 42,
        4242, 42, 42, 4234, 2424, 242, 4242, 4242, 4242, 424, 242, 424, 2, 242, 24, 24, 24, 2, 2,
        42, 42, 4, 24, 24, 2, 2, 4, 24, 2, 42, 4, 24, 24, 2, 42, 4, 24, 2, 42, 4, 4, 3, 424, 2, 4,
        24, 2, 423, 42, 4, 2, 4, 2, 42, 42, 42, 23, 42, 4, 2, 4, 2, 4, 2, 42, 4, 2, 24244, 2, 42,
        3, 422, 3432, 13, 12, 313, 1,
    ];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
