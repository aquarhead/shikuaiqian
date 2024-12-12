use std::collections::HashMap;
use std::str::FromStr;

pub fn alloc(ins: &str) -> usize {
  let input: Vec<_> = ins
    .split_ascii_whitespace()
    .map(|x| usize::from_str(x).unwrap())
    .collect();
  let mut curr: HashMap<usize, usize> = HashMap::new();
  for i in input.iter() {
    *curr.entry(*i).or_insert(0) += 1;
  }
  for _ in 0..75 {
    let mut next: HashMap<usize, usize> = HashMap::new();
    for (i, cnt) in curr.iter() {
      let mut pending: Vec<_> = Vec::new();
      if *i == 0 {
        pending.push(1);
      } else {
        let istr = i.to_string();
        if istr.len() & 1 == 0 {
          pending.push(usize::from_str(&istr[..istr.len() / 2]).unwrap());
          pending.push(usize::from_str(&istr[istr.len() / 2..]).unwrap());
        } else {
          pending.push(*i * 2024);
        }
      }
      pending.iter().for_each(|x| *next.entry(*x).or_insert(0) += cnt);
    }
    curr = next;
  }
  curr.iter().map(|(_, cnt)| cnt).sum()
}

pub fn opt(input: &str) -> usize {
  let mut curr: HashMap<_, _> = input
    .split_ascii_whitespace()
    .map(|x| (usize::from_str(x).unwrap(), 1))
    .collect();

  for _ in 0..75 {
    let mut next: HashMap<usize, usize> = HashMap::with_capacity(2 * curr.len());
    for (i, cnt) in curr.iter() {
      if *i == 0 {
        *next.entry(1).or_insert(0) += cnt
      } else {
        let istr = i.to_string();
        if istr.len() & 1 == 0 {
          *next
            .entry(usize::from_str(&istr[..istr.len() / 2]).unwrap())
            .or_insert(0) += cnt;
          *next
            .entry(usize::from_str(&istr[istr.len() / 2..]).unwrap())
            .or_insert(0) += cnt;
        } else {
          *next.entry(*i * 2024).or_insert(0) += cnt;
        }
      }
    }
    curr = next;
  }
  curr.iter().map(|(_, cnt)| cnt).sum()
}

pub fn localfn(input: &str) -> usize {
  let mut curr: HashMap<_, _> = input
    .split_ascii_whitespace()
    .map(|x| (usize::from_str(x).unwrap(), 1))
    .collect();

  for _ in 0..75 {
    let mut next: HashMap<usize, usize> = HashMap::with_capacity(2 * curr.len());
    for (i, cnt) in curr.iter() {
      let mut update = |x: usize| *next.entry(x).or_insert(0) += cnt;
      if *i == 0 {
        update(1);
      } else {
        let istr = i.to_string();
        if istr.len() & 1 == 0 {
          update(usize::from_str(&istr[..istr.len() / 2]).unwrap());
          update(usize::from_str(&istr[istr.len() / 2..]).unwrap());
        } else {
          update(*i * 2024);
        }
      }
    }
    curr = next;
  }
  curr.iter().map(|(_, cnt)| cnt).sum()
}

struct Blink<I: Iterator<Item = (usize, usize)>> {
  counts: I,
  right: Option<(usize, usize)>,
}

impl<I: Iterator<Item = (usize, usize)>> Blink<I> {
  fn from(counts: I) -> Self {
    Self { counts, right: None }
  }
}

impl<I: Iterator<Item = (usize, usize)>> Iterator for Blink<I> {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(r) = self.right {
      self.right = None;
      return Some(r);
    }
    if let Some((stone, count)) = self.counts.next() {
      if stone == 0 {
        Some((1, count))
      } else {
        let istr = stone.to_string();
        if istr.len() & 1 == 0 {
          self.right = Some((usize::from_str(&istr[..istr.len() / 2]).unwrap(), count));
          Some((usize::from_str(&istr[istr.len() / 2..]).unwrap(), count))
        } else {
          Some((stone * 2024, count))
        }
      }
    } else {
      None
    }
  }
}

pub fn makeiter(input: &str) -> usize {
  let curr: HashMap<_, _> = input
    .split_ascii_whitespace()
    .map(|x| (usize::from_str(x).unwrap(), 1))
    .collect();

  (0..75)
    .fold(curr, |acc, _| {
      let cap = 2 * acc.len();
      Blink::from(acc.into_iter()).fold(HashMap::with_capacity(cap), |mut acc, (x, cnt)| {
        *acc.entry(x).or_insert(0) += cnt;
        acc
      })
    })
    .values()
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn same() {
    assert_eq!(
      alloc("5 62914 65 972 0 805922 6521 1639064"),
      opt("5 62914 65 972 0 805922 6521 1639064")
    );
    assert_eq!(
      alloc("5 62914 65 972 0 805922 6521 1639064"),
      localfn("5 62914 65 972 0 805922 6521 1639064")
    );
    assert_eq!(
      alloc("5 62914 65 972 0 805922 6521 1639064"),
      makeiter("5 62914 65 972 0 805922 6521 1639064")
    );
  }
}
