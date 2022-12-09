use competition_io::input_new;

use std::collections::vec_deque::VecDeque;

fn main() {
    let mut inp = input_new();

    let mut queue = VecDeque::new();

    for _ in 0..14 {
        queue.push_back(inp.next_char_wsf());
    }

    let mut i = 14;
    'outer : loop {
        queue.pop_front();
        queue.push_back(inp.next_char_wsf());
        i += 1;

        for n in 0..14 {
            for m in n+1..14 {
                if queue[n] == queue[m] {
                    continue 'outer
                };
            }
        }

        println!("Index is {i}");
        break
        
    }
}
