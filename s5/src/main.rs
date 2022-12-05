use std::iter::from_fn;

use competition_io::input_new;

fn main() {
    let mut inp = input_new();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    'outer : loop {
        inp.next_char();

        for i in 0.. {
            let c = inp.next_char();

            if c == '\n' {
                break;
            }
            if c == '1' {
                break 'outer;
            }

            if stacks.len() <= i {
                stacks.extend(from_fn(|| Some(Vec::new())).take(i - stacks.len() + 1));
            }

            if c != ' ' {
                stacks[i].push(c);
            }

            for _ in 0..3 {
                let t = inp.next_char();
                if t == '\n' {
                    continue 'outer;
                }
            }

        }
    }

    for _ in 1..stacks.len() {
        inp.next::<String>();
    }

    for i in 0..stacks.len() {
        stacks[i].reverse();
    }

    loop {
        if &inp.next::<String>() == "q" {
            break
        }
        let cnt : usize = inp.next();
        inp.next::<String>();
        let from : usize = inp.next::<usize>() -1;
        inp.next::<String>();
        let to : usize = inp.next::<usize>() -1;

        for _ in 0..cnt {
            let moved = stacks[from].pop().unwrap();
            stacks[to].push(moved);
        }
    }

    let mut string = String::new();
    for stack in stacks {
        string.push(*stack.last().unwrap())
    }

    println!("{string}");
}

