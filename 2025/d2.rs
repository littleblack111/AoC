use aoc::fetch::fetch;

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

    fn parse_section_part1(num: usize) -> usize {
        let s = num.to_string();
        let len = s.len();
        for substr_len in 1..=len / 2 {
            if len % substr_len == 0 {
                let first = &s[0..substr_len];
                if s.as_bytes()
                    .chunks(substr_len)
                    .all(|chunk| chunk == first.as_bytes())
                {
                    return num;
                }
            }
        }
        0
    }

    fn parse_section_part2(num: usize) -> usize {
        let s = num.to_string();
        if s.len() % 2 != 0 {
            return 0;
        }
        if s[0..s.len() / 2] == s[s.len() / 2..s.len()] {
            return num;
        }
        0
    }

    fn parse(self, part2: bool) -> usize {
        let mut ans = 0;
        for i in self.first..=self.last {
            ans += if part2 {
                Self::parse_section_part2(i)
            } else {
                Self::parse_section_part1(i)
            };
        }
        ans
    }
}

fn run(part2: bool) {
    let input = fetch(
        2, ',',
    );
    let mut ans = 0;

    for s in input {
        let s = s.replace(
            '\n', "",
        );
        if s.is_empty() {
            continue;
        }
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
        .parse(part2);
    }
    println!("{ans}");
}

pub fn part1() {
    run(false);
}

pub fn part2() {
    run(true);
}
