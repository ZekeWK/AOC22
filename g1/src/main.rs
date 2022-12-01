use competition_io::*;

fn main() {
    let mut inp = input_new();

    let mut max = [0u64; 3];

    loop {
        let mut sum = 0;
        while let Some(val) = inp.next_on_line::<u64>() {
            sum += val;
            inp.next_on_line::<u64>();
        }

        if sum == 0 {
            break
        }

        for i in 0..3 {
            if sum > max[i] {
                std::mem::swap(&mut max[i], &mut sum);
            }
        }
    }

    let max_sum : u64 = max.iter().sum();

    println!("Max is {max_sum}")
}
