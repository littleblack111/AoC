use aoc::fetch::fetch;

#[derive(Debug)]
struct Ring {
    inner: usize,
    counter: usize,
}

impl Ring {
    fn new() -> Self {
        Self {
            inner: 50,
            counter: Default::default(),
        }
    }

    const MAX: usize = 99;

    fn left(&mut self, num: usize, count_each_step: bool) {
        for _ in 0..num {
            if self.inner == 0 {
                self.inner = Self::MAX;
            } else {
                self.inner -= 1;
            }
            if count_each_step {
                self.check_counter();
            }
        }
        if !count_each_step {
            self.check_counter();
        }
    }

    fn right(&mut self, num: usize, count_each_step: bool) {
        for _ in 0..num {
            if self.inner == Self::MAX {
                self.inner = 0;
            } else {
                self.inner += 1;
            }
            if count_each_step {
                self.check_counter();
            }
        }
        if !count_each_step {
            self.check_counter();
        }
    }

    fn check_counter(&mut self) {
        if self.inner == 0 {
            self.counter += 1;
        }
    }
}

fn solve(count_each_step: bool) -> usize {
    let mut ring = Ring::new();
    let input = fetch(
        1, '\n',
    );
    for i in input {
        if i.is_empty() {
            continue;
        }
        let dir = i
            .chars()
            .next()
            .unwrap();
        let num = i[1..]
            .parse()
            .unwrap();

        match dir {
            'L' => ring.left(
                num,
                count_each_step,
            ),
            'R' => ring.right(
                num,
                count_each_step,
            ),
            _ => {}
        }
    }
    ring.counter
}

pub fn part1() -> usize {
    solve(false)
}

pub fn part2() -> usize {
    solve(true)
}
