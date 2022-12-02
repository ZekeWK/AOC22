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
            y + 1 + (((3 + y - x)% 3) == 1) as usize *6 + (x==y) as usize * 3
        })
        .sum() ;

    println!("The sum is: {sum}");
}
