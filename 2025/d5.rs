use std::collections::HashSet;

use aoc::fetch::fetch;

#[derive(PartialEq)]
enum Type {
    Spoiled,
    Fresh,
}

struct Ingredient {
    inner: usize,
    kind: Option<Type>,
}

impl Ingredient {
    fn new(inner: usize) -> Self {
        Self {
            inner,
            kind: None,
        }
    }
}

struct Db {
    ranges: Vec<(
        usize,
        usize,
    )>,
    ingredient: Vec<Ingredient>,
}

impl Db {
    fn new(
        ranges: Vec<(
            usize,
            usize,
        )>,
        ingredient: Vec<Ingredient>,
    ) -> Self {
        Self {
            ranges,
            ingredient,
        }
    }

    fn assign_type(&mut self) {
        // for (start, end) in &self.ranges {
        //     for i in *start..=*end {
        //         for j in &mut self.ingredient {
        //             if j.inner == i {
        //                 j.kind = Some(Type::Fresh);
        //             }
        //         }
        //     }
        // }
        let len = self
            .ingredient
            .len();
        for (index, i) in self
            .ingredient
            .iter_mut()
            .enumerate()
        {
            println!(
                "{}",
                index / len * 100
            );
            for (start, end) in &self.ranges {
                if i.inner >= *start && i.inner <= *end {
                    i.kind = Some(Type::Fresh);
                    break;
                }
            }
        }
    }

    fn get_fresh(&self) -> usize {
        self.ingredient
            .iter()
            .fold(
                0,
                |mut prev, i| {
                    if let Some(i) = &i.kind
                        && *i == Type::Fresh
                    {
                        prev += 1;
                    }
                    prev
                },
            )
    }

    fn get_all_ranges_sum(&self) -> usize {
        let mut ids: HashSet<usize> = HashSet::new();
        for (start, end) in &self.ranges {
            for i in *start..=*end {
                ids.insert(i);
            }
        }
        ids.len()
    }
}

pub fn part1() {
    let input = fetch(
        5, '\n',
    );
    let mut input = input.split(|s| s.is_empty());
    // let (mut raw_ranges, mut raw_ing) = input.split_at(
    //     input
    //         .iter()
    //         .position(|s| s.is_empty())
    //         .unwrap(),
    // );
    let (mut raw_ranges, mut raw_ing) = (
        input
            .next()
            .unwrap()
            .to_vec(),
        input
            .next()
            .unwrap()
            .to_vec(),
    );
    println!(
        "{:#?} {:#?}",
        raw_ranges, raw_ing
    );
    // let (mut raw_ranges, mut raw_ing) = (
    //     raw_ranges.to_vec(),
    //     raw_ing.to_vec(),
    // );
    // println!(
    //     "{:#?} {:#?}",
    //     raw_ranges, raw_ing
    // );
    let mut ingredient = Vec::new();
    for i in &raw_ing {
        println!("{i}");
        ingredient.push(
            Ingredient::new(
                i.parse()
                    .unwrap(),
            ),
        );
    }
    let mut ranges: Vec<(
        usize,
        usize,
    )> = Vec::new();
    for i in &raw_ranges {
        let mut split = i.split('-');
        println!("{split:#?}");
        ranges.push(
            (
                split
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
                split
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
        );
    }
    let mut db = Db::new(
        ranges, ingredient,
    );
    db.assign_type();

    println!(
        "{}",
        db.get_fresh()
    );
}

pub fn part2() {
    let input = fetch(
        5, '\n',
    );
    let mut input = input.split(|s| s.is_empty());
    // let (mut raw_ranges, mut raw_ing) = input.split_at(
    //     input
    //         .iter()
    //         .position(|s| s.is_empty())
    //         .unwrap(),
    // );
    let (mut raw_ranges, mut raw_ing) = (
        input
            .next()
            .unwrap()
            .to_vec(),
        input
            .next()
            .unwrap()
            .to_vec(),
    );
    println!(
        "{:#?} {:#?}",
        raw_ranges, raw_ing
    );
    // let (mut raw_ranges, mut raw_ing) = (
    //     raw_ranges.to_vec(),
    //     raw_ing.to_vec(),
    // );
    // println!(
    //     "{:#?} {:#?}",
    //     raw_ranges, raw_ing
    // );
    let mut ingredient = Vec::new();
    for i in &raw_ing {
        println!("{i}");
        ingredient.push(
            Ingredient::new(
                i.parse()
                    .unwrap(),
            ),
        );
    }
    let mut ranges: Vec<(
        usize,
        usize,
    )> = Vec::new();
    for i in &raw_ranges {
        let mut split = i.split('-');
        println!("{split:#?}");
        ranges.push(
            (
                split
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
                split
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
        );
    }
    let mut db = Db::new(
        ranges, ingredient,
    );

    println!(
        "{}",
        db.get_all_ranges_sum()
    );
}
