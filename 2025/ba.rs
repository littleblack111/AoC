use aoc::{fetch::fetch, read::read};

struct Checker {
    first: usize,
    last: usize,
}

impl Checker {
    fn new(first: usize, last: usize) -> Result<Self, ()> {
        if first
            .to_string()
            .chars()
            .next()
            .unwrap()
            == '0'
        {
            return Err(());
        }

        Ok(
            Self {
                first,
                last,
            },
        )
    }

    fn parse_section_naive(num: usize) -> usize {
        let mut prevs = String::new();
        let mut ans = 0;
        for c in num
            .to_string()
            .chars()
        {
            if prevs.is_empty() || prevs.contains(c) {
                prevs.push(c);
            }
            if !prevs.contains(c) {
                prevs.clear()
            }
            if prevs.len() >= 2
                && (num
                    .to_string()
                    .len()
                    == 2
                    || !prevs.contains(c))
            {
                ans += prevs
                    .parse::<usize>()
                    .unwrap();

                prevs.clear();
            }
        }
        ans
    }

    // fn parse_section(num: usize) -> usize {
    //     let mut ans = 0;
    //     for (i, c) in num
    //         .to_string()
    //         .chars()
    //         .enumerate()
    //     {
    //         for j in i..=num
    //             .to_string()
    //             .len()
    //         {
    //             for k in (j..=num
    //                 .to_string()
    //                 .len()
    //                 / 2)
    //                 .rev()
    //             {
    //                 ans += if num.to_string()[j..=k] == num.to_string()[k..=k +
    // (j..k).len()] {                     println!(
    //                         "{} {j} {k}",
    //                         num.to_string()[j..k].to_string()
    //                     );
    //                     num.to_string()[j..=k]
    //                         .parse::<usize>()
    //                         .unwrap()
    //                 } else {
    //                     0
    //                 }
    //             }
    //         }
    //     }
    //     ans
    // }

    fn parse_section(num: usize) -> usize {
        let s = num.to_string();
        if s.len() % 2 != 0 {
            return 0;
        }
        if s[0..s.len() / 2] == s[s.len() / 2..s.len()] {
            return num;
        }
        0
    }

    fn parse(self) -> usize {
        let mut ans = 0;
        for i in self.first..=self.last {
            ans += Self::parse_section(i);
        }
        ans
    }
}

pub fn main() {
    let input = fetch(
        2, ',',
    );
    // let input = read(
    //     "b", ',',
    // );
    let mut ans = 0;

    println!(
        "{}",
        Checker::new(
            998, 1012,
        )
        .unwrap()
        .parse()
    );
    println!(
        "{}",
        Checker::new(
            11, 22,
        )
        .unwrap()
        .parse()
    );
    // return;
    for s in input {
        let s = s.replace(
            '\n', "",
        );
        let split = s.split('-');
        let first = split
            .clone()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let last = split
            .clone()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        ans += &Checker::new(
            first, last,
        )
        .unwrap()
        .parse();
    }
    println!("{ans}");
}
