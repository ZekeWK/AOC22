use competition_io::*;

fn main() {
    let mut inp = input_new();

    let mut max = 0;
    loop {
        let mut sum = 0;
        while let Some(val) = inp.next_on_line::<u64>() {
            sum += val;
            inp.next_on_line::<u64>();
        }

        if sum == 0 {
            break
        }
        if sum >= max {
            max = sum;
        }
    }

    println!("Max is {max}")
}
