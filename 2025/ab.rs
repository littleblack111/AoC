use aoc::read::read;

#[derive(Debug)]
struct Ring {
    inner: usize,
    counter: usize,
}

impl Ring {
    fn new() -> Self {
        Self {
            inner: 50, // start in 50
            counter: Default::default(),
        }
    }

    const MAX: usize = 99;
    // minus
    fn left(&mut self, num: usize) {
        for _ in 0..num {
            if self.inner == 0 {
                self.inner = Self::MAX;
            } else {
                self.inner -= 1;
            }
            self.counter();
        }
    }

    // plus
    fn right(&mut self, num: usize) {
        for _ in 0..num {
            if self.inner == Self::MAX {
                self.inner = 0;
            } else {
                self.inner += 1;
            }
            self.counter();
        }
    }

    fn counter(&mut self) {
        if self.inner == 0 {
            self.counter += 1;
        }
    }
}

pub fn main() {
    let mut ring = Ring::new();
    let input = read("a");
    for i in input {
        let dir = i
            .chars()
            .next()
            .unwrap();
        let num = i[1..]
            .parse()
            .unwrap();

        match dir {
            'L' => ring.left(num),
            'R' => ring.right(num),
            _ => {}
        }
        println!(
            "{}: {:#?}",
            num, ring
        )
    }
    println!(
        "Result: {}",
        ring.counter
    );
}
