use std::ops::{Deref, DerefMut};

use aoc::{fetch::fetch, read::read};

#[derive(PartialEq, Clone, Debug)]
enum Symbol {
    None,
    Paper,
}

#[derive(Debug)]
struct Finder {
    map: Map,
    x: usize,
    y: usize,
}

impl Finder {
    fn new(map: Map) -> Self {
        Self {
            map,
            x: usize::MAX,
            y: Default::default(),
        }
    }

    fn next_paper(&mut self) -> bool {
        for i in self.y
            ..self
                .map
                .len()
        {
            let x = if i == self.y {
                self.x
                    .wrapping_add(1)
            } else {
                0
            };
            for j in x..self.map[i].len() {
                match &self.map[i][j] {
                    Symbol::Paper => {
                        self.x = j;
                        self.y = i;
                        return true;
                    }
                    Symbol::None => {}
                }
            }
        }
        false
    }

    fn get_prox(&self) -> usize {
        let mut total = 0;
        for y in -1i32..=1 {
            for x in -1i32..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                let ny = self
                    .y
                    .wrapping_add(y as usize);
                let nx = self
                    .x
                    .wrapping_add(x as usize);
                if ny
                    < self
                        .map
                        .len()
                    && nx < self.map[ny].len()
                    && self.map[ny][nx] == Symbol::Paper
                {
                    total += 1;
                }
            }
        }
        total
    }
}

#[derive(Clone, Debug)]
struct Map {
    inner: Vec<Vec<Symbol>>,
}

impl Map {
    fn new(inner: Vec<Vec<Symbol>>) -> Self {
        Self {
            inner,
        }
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for Map {
    type Target = Vec<Vec<Symbol>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn part2() {
    let input = fetch(
        4, '\n',
    );
    // let input: Vec<String> = read(
    //     "e", '\n',
    // );
    // .into_iter()
    // .filter(|s| !s.is_empty())
    // .collect();
    let mut vec: Vec<Vec<Symbol>> = vec![Vec::new(); input.len()];
    for (i, s) in input
        .iter()
        .enumerate()
    {
        for c in s.chars() {
            vec[i].push(
                match c {
                    '.' => Symbol::None,
                    '@' => Symbol::Paper,
                    _ => panic!(),
                },
            );
        }
    }
    let map = Map::new(vec);
    let mut finder = Finder::new(map);
    let mut res = 0;
    let mut ires = 0;
    loop {
        if !finder.next_paper() {
            if ires == 0 {
                break;
            } else {
                res += ires;
                ires = 0;
                finder.x = usize::MAX;
                finder.y = 0;
                continue;
            }
        }
        println!(
            "{} at {},{}",
            finder.get_prox(),
            finder.x,
            finder.y
        );
        if finder.get_prox() < 4 {
            finder.map[finder.y][finder.x] = Symbol::None;
            ires += 1;
        }
    }
    println!("{res}");
}

pub fn part1() {
    // let input = fetch(
    //     4, '\n',
    // );
    let input: Vec<String> = read(
        "e", '\n',
    );
    // .into_iter()
    // .filter(|s| !s.is_empty())
    // .collect();
    let mut vec: Vec<Vec<Symbol>> = vec![Vec::new(); input.len()];
    for (i, s) in input
        .iter()
        .enumerate()
    {
        for c in s.chars() {
            vec[i].push(
                match c {
                    '.' => Symbol::None,
                    '@' => Symbol::Paper,
                    _ => panic!(),
                },
            );
        }
    }
    let map = Map::new(vec);
    let mut finder = Finder::new(map);
    let mut res = 0;
    loop {
        if !finder.next_paper() {
            break;
        }
        println!(
            "{} at {},{}",
            finder.get_prox(),
            finder.x,
            finder.y
        );
        if finder.get_prox() < 4 {
            res += 1
        }
    }
    // We skipped the first since we directly x+1 when going into the loop,
    // could prolly fix it via init x: usize::MAX then just wrapping add, but
    // too lazy
    println!("{res}");
}
