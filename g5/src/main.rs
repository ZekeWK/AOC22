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

        let (from, to) = take_two_mut(&mut stacks, from, to);

        let from_len = from.len();
        to.extend_from_slice(&from[from_len - cnt .. from_len]);
        for _ in 0..cnt {
            from.pop();
        }
    }

    let mut string = String::new();
    for stack in stacks {
        string.push(*stack.last().unwrap())
    }

    println!("{string}");
}

fn take_two_mut<T>(slice : &mut [T], a : usize, b : usize) -> (&mut T, &mut T) {
    if a < b {
        let (sa, sb) = slice.split_at_mut(a+1);
        (&mut sa[a], &mut sb[ b -a - 1])
    } else if a > b {
        let (sb, sa) = slice.split_at_mut(b+1);
        (&mut sa[ a -b - 1], &mut sb[b])
    }
    else {
        panic!()
    }
}
