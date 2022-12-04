use competition_io::input_new;
use bitvec::BitVec;

const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut inp = input_new();

    let mut sum = 0;

    loop {
        let string = inp.next::<String>();
        let string = string.trim();
        let len = string.len();
        if string == "q" {
            break
        }

        let mut iter = string.chars();
        
        let mut sack1 = BitVec::zeroes(52);
        let mut sack2 = BitVec::zeroes(52);

        for _ in 0..len/2 {
            let char = iter.next().unwrap();
            let priority : usize = ALPHABET.chars().position(|c| c == char).unwrap();

            sack1.set(priority, true).unwrap();
        }

        for _ in 0..len/2 {
            let char = iter.next().unwrap();
            let priority : usize = ALPHABET.chars().position(|c| c == char).unwrap();

            sack2.set(priority, true).unwrap();
        }

        for i in 0..52 {
            if sack1.get(i).unwrap() && sack2.get(i).unwrap() {
                sum += i + 1;
                break;
            }
        }
    }

    println!("The sum is {sum}");
}
