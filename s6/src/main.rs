use std::iter::from_fn;
use competition_io::input_new;
use itertools::Itertools;

fn main() {
    let mut inp = input_new();

    let index = from_fn(|| Some(inp.next_char()))
    .tuple_windows::<(char, char, char, char)>()
    .take_while(|(a, b, c, d)| ![a,b,c,d].iter().all_unique())
    .count() + 4;

    println!("The index is {index}");

}
