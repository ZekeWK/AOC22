use std::iter::from_fn;

use competition_io::input_new;

fn main() {
    let mut inp = input_new();

    let sum : usize = from_fn(|| Some((inp.next_char_wsf(), inp.next_char_wsf())))
        .take_while(|(x, y)| *x != 'q' && *y != 'q')
        .map(|(x, y)| {
            (
                ['A', 'B', 'C'].iter().position(|a| *a==x).unwrap(),
                ['X', 'Y', 'Z'].iter().position(|a| *a ==y).unwrap()
            )
        })
        .map(|(x, y)| {
            y * 3 + ((x + 5 + y)%3) + 1
        })
        .sum() ;

    println!("The sum is: {sum}");
}
