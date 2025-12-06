use aoc::{fetch::fetch, read::read};

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operator::Plus,
            "*" => Operator::Multiply,
            _ => panic!(
                "Unknown operator: {}",
                s
            ),
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operator::Plus),
            '*' => Ok(Operator::Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Expr {
    nums: Vec<usize>,
    ops: Operator,
}

impl Expr {
    fn new(nums: Vec<usize>, ops: Operator) -> Self {
        Self {
            nums,
            ops,
        }
    }

    fn calc(&self) -> usize {
        match self.ops {
            Operator::Plus => self
                .nums
                .iter()
                .sum(),
            Operator::Multiply => self
                .nums
                .iter()
                .product(),
        }
    }

    fn calc_rtl(&self) -> usize {
        // appearently they need original raw spaces on which digit is more significant,
        // meaning i need to refactor everything, so i give up
        // let len = self
        //     .nums
        //     .iter()
        //     .fold(
        //         0,
        //         |prev, x| *x.max(&prev),
        //     )
        //     .to_string()
        //     .len();
        let mut nums = self
            .nums
            .clone();
        // left pad
        // for i in nums.iter_mut() {
        //     *i = format!("{i:0>len$}")
        //         .parse()
        //         .unwrap();
        // }
        // println!(
        //     "{:?}",
        //     nums
        // );

        let mut processed_num = Vec::default();
        for i in 0..len {
            let mut ans = String::new();
            for n in &nums {
                for (j, c) in n
                    .to_string()
                    .chars()
                    .rev()
                    .enumerate()
                {
                    if j == i {
                        ans += &c.to_string();
                    }
                }
            }
            println!("{ans}");
            processed_num.push(
                ans.parse()
                    .unwrap(),
            );
        }
        let a = processed_num
            .iter()
            .fold(
                match self.ops {
                    Operator::Plus => 0,
                    Operator::Multiply => 1,
                },
                |acc, x| match self.ops {
                    Operator::Plus => acc + x,
                    Operator::Multiply => acc * x,
                },
            );
        println!("{a}");
        a
    }
}

struct Calc {
    exprs: Vec<Expr>,
}

impl Calc {
    fn new(exprs: Vec<Expr>) -> Self {
        Self {
            exprs,
        }
    }

    fn calc_sum(&self) -> usize {
        self.exprs
            .iter()
            .fold(
                0,
                |sum, expr| sum + expr.calc(),
            )
    }

    fn calc_rtl_sum(&self) -> usize {
        self.exprs
            .iter()
            .fold(
                0,
                |sum, expr| sum + expr.calc_rtl(),
            )
    }
}

fn parse(input: Vec<String>) -> Vec<Expr> {
    let len = input[0]
        .split_whitespace()
        .collect::<Vec<_>>()
        .len();
    let mut exprs = Vec::new();
    for i in 0..len {
        let mut nums = Vec::new();
        for (j, s) in input
            .iter()
            .enumerate()
        {
            if j == input.len() - 1 {
                continue;
            }
            for (k, m) in s
                .split_whitespace()
                .enumerate()
            {
                if k == i {
                    nums.push(
                        m.parse()
                            .unwrap(),
                    );
                }
            }
        }
        exprs.push(
            Expr::new(
                nums,
                input[input.len() - 1]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .nth(i)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
        );
    }
    exprs
}

pub fn part1() {
    let input = read(
        "f", "\n",
    );
    // let input = fetch(
    //     6, '\n',
    // );
    println!(
        "{:#?}",
        Calc::new(parse(input)).calc_rtl_sum()
    );
}
