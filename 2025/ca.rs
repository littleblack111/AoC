use aoc::{fetch::fetch, joltage::Joltage, read::read};

struct JoltageBank {
    inner: Vec<Joltage>, // optimization: could be fixzed size slice
}

impl JoltageBank {
    fn new(inner: Vec<Joltage>) -> Self {
        Self {
            inner,
        }
    }

    fn get_biggest_two(&mut self) -> usize {
        let mut res = 0;
        // Try all pairs of positions (i, j) where i < j
        // The resulting number is formed by digit at i followed by digit at j
        for i in 0..self
            .inner
            .len()
        {
            for j in (i + 1)
                ..self
                    .inner
                    .len()
            {
                let max1 = *self.inner[i] as usize;
                let max2 = *self.inner[j] as usize;
                let value = max1 * 10 + max2;
                if value > res {
                    res = value;
                }
            }
        }
        res
    }
}

pub fn main() {
    let input = fetch(
        3, '\n',
    );
    // let vec: Vec<Joltage> = "987654321111111"
    //     .chars()
    //     .map(
    //         |c| {
    //             Joltage::parse(
    //                 c.to_digit(10)
    //                     .unwrap() as u8,
    //             )
    //             .unwrap()
    //         },
    //     )
    //     .collect();
    // println!(
    //     "{:#?}",
    //     JoltageBank::new(vec).get_max(2)
    // );
    // let input = read(
    //     "c", '\n',
    // );
    let mut res = 0;
    for i in input {
        if i.is_empty() {
            break;
        }
        let mut vec = Vec::new();
        for c in i.chars() {
            vec.push(
                Joltage::parse(
                    c.to_digit(10)
                        .unwrap() as u8,
                )
                .unwrap(),
            );
        }
        res += JoltageBank::new(vec).get_biggest_two();
    }
    println!("{res}");
}
