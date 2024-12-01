use std::io;

use itertools::Itertools;

fn p1(a: &[u32], b: &[u32]) -> u32 {
    a.iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b))
}

fn p2(a: &[u32], b: &[u32]) -> u32 {
    a.iter()
        .map(|&num| {
            let mul = if let Ok(in_b) = b.binary_search(&num) {
                let (mut ubound, mut lbound) = (in_b, in_b);

                while num == b[ubound] {
                    ubound += 1;
                }

                while num == b[lbound] {
                    lbound -= 1;
                }

                (ubound - lbound) as u32 - 1
            } else {
                0
            };

            num * mul
        })
        .sum()
}

pub fn day1() -> io::Result<(u32, u32)> {
    let inp = std::fs::read_to_string("inputs/day1")?;
    let (mut a, mut b) = inp
        .lines()
        .map(|line| {
            line.split("   ")
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<(Vec<_>, Vec<_>)>();

    a.sort();
    b.sort();

    Ok((p1(&a, &b), p2(&a, &b)))
}
