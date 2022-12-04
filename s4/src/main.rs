use competition_io::input_new;

fn main() {
    let mut inp = input_new();

    let mut count = 0;

    loop {
        let line : String = inp.next();
        if line == "q" {
            break
        }

        let (range1, range2) = line.split_once(",").unwrap();

        let (a1, b1) = range1.split_once("-").unwrap();
        let a1 : u64 = a1.parse().unwrap();
        let b1 : u64 = b1.parse().unwrap();

        let (a2, b2) = range2.split_once("-").unwrap();
        let a2 : u64 = a2.parse().unwrap();
        let b2 : u64 = b2.parse().unwrap();

        if (a1 <= a2 && b1 >= b2) || (a2 <= a1 && b2 >= b1) {
            count += 1;
        }
    }

    println!("Count is {count}");
}
