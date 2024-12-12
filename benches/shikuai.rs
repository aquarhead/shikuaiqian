use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

// use shikuaiqian::*;

fn alloc(c: &mut Criterion) {
  c.bench_function("alloc", |b| {
    b.iter(|| shikuaiqian::alloc(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

fn opt(c: &mut Criterion) {
  c.bench_function("opt", |b| {
    b.iter(|| shikuaiqian::opt(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

fn localfn(c: &mut Criterion) {
  c.bench_function("localfn", |b| {
    b.iter(|| shikuaiqian::localfn(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

fn makeiter(c: &mut Criterion) {
  c.bench_function("makeiter", |b| {
    b.iter(|| shikuaiqian::makeiter(black_box("5 62914 65 972 0 805922 6521 1639064")))
  });
}

criterion_group!(benches, opt, localfn, makeiter, alloc);
criterion_main!(benches);
