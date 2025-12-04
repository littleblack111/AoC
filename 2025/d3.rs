use aoc::{fetch::fetch, joltage::Joltage, read::read};

struct JoltageBank {
    inner: Vec<Joltage>,
}

impl JoltageBank {
    fn new(inner: Vec<Joltage>) -> Self {
        Self {
            inner,
        }
    }

    fn get_biggest_two(&mut self) -> usize {
        let mut res = 0;
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

    fn get_biggest_12(&mut self) -> usize {
        let digits: Vec<usize> = self
            .inner
            .iter()
            .map(|j| **j as usize)
            .collect();
        let n = digits.len();
        let k = 12;
        let to_remove = n - k;

        // Use monotonic stack to find largest number by removing `to_remove` digits
        let mut stack: Vec<usize> = Vec::new();
        let mut removed = 0;

        for &digit in &digits {
            while removed < to_remove
                && !stack.is_empty()
                && *stack
                    .last()
                    .unwrap()
                    < digit
            {
                stack.pop();
                removed += 1;
            }
            stack.push(digit);
        }

        // If we haven't removed enough, remove from the end
        while stack.len() > k {
            stack.pop();
        }

        let mut res = 0;
        for val in stack {
            res = res * 10 + val;
        }
        res
    }
}

pub fn part1() {
    let input = fetch(
        3, '\n',
    );
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

pub fn part2() {
    // let input = fetch(3, '\n');
    let input = read(
        "d", '\n',
    );
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
        res += JoltageBank::new(vec).get_biggest_12();
    }
    println!("{res}");
}
