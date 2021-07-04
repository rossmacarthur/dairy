use criterion::{black_box, criterion_group, Criterion};

use beef::lean::Cow as BeefCow;
use dairy::Cow as DairyCow;
use std::borrow::Cow as StdCow;

fn beef_cow_to_beef_cow(input: BeefCow<str>) -> BeefCow<str> {
    if input.starts_with("https://") {
        input
    } else {
        let mut o = input.into_owned();
        o.insert_str(0, "https://");
        BeefCow::owned(o)
    }
}

fn dairy_cow_to_dairy_cow(input: DairyCow<str>) -> DairyCow<str> {
    if input.starts_with("https://") {
        input
    } else {
        let mut o = input.into_owned();
        o.insert_str(0, "https://");
        DairyCow::owned(o)
    }
}

fn std_cow_to_std_cow(mut input: StdCow<str>) -> StdCow<str> {
    if !input.starts_with("https://") {
        let o = input.to_mut();
        o.insert_str(0, "https://");
    }
    input
}

fn maybe_modify(c: &mut Criterion) {
    c.bench_function("maybe/modify/beef/borrowed", |b| {
        b.iter(|| beef_cow_to_beef_cow(black_box(BeefCow::borrowed("test"))))
    });
    c.bench_function("maybe/modify/dairy/borrowed", |b| {
        b.iter(|| dairy_cow_to_dairy_cow(black_box(DairyCow::borrowed("test"))))
    });
    c.bench_function("maybe/modify/std/borrowed", |b| {
        b.iter(|| std_cow_to_std_cow(black_box(StdCow::Borrowed("test"))))
    });

    c.bench_function("maybe/modify/beef/owned", |b| {
        b.iter(|| beef_cow_to_beef_cow(black_box(BeefCow::owned(String::from("test")))))
    });
    c.bench_function("maybe/modify/dairy/owned", |b| {
        b.iter(|| dairy_cow_to_dairy_cow(black_box(DairyCow::owned(String::from("test")))))
    });
    c.bench_function("maybe/modify/std/owned", |b| {
        b.iter(|| std_cow_to_std_cow(black_box(StdCow::Owned(String::from("test")))))
    });

    c.bench_function("maybe/noop/beef/borrowed", |b| {
        b.iter(|| beef_cow_to_beef_cow(black_box(BeefCow::borrowed("https://test"))))
    });
    c.bench_function("maybe/noop/dairy/borrowed", |b| {
        b.iter(|| dairy_cow_to_dairy_cow(black_box(DairyCow::borrowed("https://test"))))
    });
    c.bench_function("maybe/noop/std/borrowed", |b| {
        b.iter(|| std_cow_to_std_cow(black_box(StdCow::Borrowed("https://test"))))
    });

    c.bench_function("maybe/noop/beef/owned", |b| {
        b.iter(|| beef_cow_to_beef_cow(black_box(BeefCow::owned(String::from("https://test")))))
    });
    c.bench_function("maybe/noop/dairy/owned", |b| {
        b.iter(|| dairy_cow_to_dairy_cow(black_box(DairyCow::owned(String::from("https://test")))))
    });
    c.bench_function("maybe/noop/std/owned", |b| {
        b.iter(|| std_cow_to_std_cow(black_box(StdCow::Owned(String::from("https://test")))))
    });
}

criterion_group!(benches, maybe_modify);
