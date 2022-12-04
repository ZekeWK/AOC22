use competition_io::input_new;
use bitvec::BitVec;

const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut inp = input_new();

    let mut sum = 0;

    loop {
        let mut get_sack = || {
            let string = inp.next::<String>();
            if string == "q" {
                return None
            }

            let mut sack = BitVec::zeroes(52);

            for char in string.chars() {
                let priority : usize = ALPHABET.chars().position(|c| c == char).unwrap();

                sack.set(priority, true).unwrap();
            }
            Some(sack)
        };

        let Some(sack1) = get_sack() else {break};
        let Some(sack2) = get_sack() else {break};
        let Some(sack3) = get_sack() else {break};

        for i in 0..52 {
            if sack1.get(i).unwrap() && sack2.get(i).unwrap() && sack3.get(i).unwrap() {
                sum += i + 1;
                break;
            }
        }
    }

    println!("The sum is {sum}");
}
