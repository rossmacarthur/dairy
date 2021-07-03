use criterion::{black_box, criterion_group, Criterion};

use beef::lean::Cow as BeefCow;
use dairy::Cow as DairyCow;
use std::borrow::Cow as StdCow;

const NTH_WORD: usize = 2;
static TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam ut ipsum quis orci sagittis consectetur in vel nisi. Fusce eu magna condimentum turpis posuere vulputate id eu massa. Sed id rhoncus ligula. Donec aliquet et odio quis bibendum. Vivamus quis tortor neque. Sed ac malesuada eros. Sed sed varius nunc. Fusce pretium ultrices dictum. Cras ipsum turpis, maximus eu eros auctor, varius tincidunt nibh. Pellentesque elementum tincidunt finibus.

Ut porta finibus eros, quis commodo magna aliquet at. Vestibulum eu arcu luctus, porttitor diam at, cursus ligula. Ut nec massa id ex rhoncus dignissim. Aliquam efficitur id augue sit amet tincidunt. Phasellus convallis mollis scelerisque. Vivamus vitae dui id augue mollis ornare. Etiam nec nibh nec ante accumsan lobortis vitae pretium lorem. Donec nulla ante, posuere laoreet ipsum vitae, laoreet pellentesque sapien. Vestibulum eget metus auctor, volutpat lectus eu, iaculis lectus. Duis eget diam vel libero bibendum finibus sed eu sapien. Sed eget ultrices nunc, non cursus nisi. Suspendisse quis dictum nibh. Cras id massa mollis, convallis quam et, finibus massa.

Cras eu semper justo. Pellentesque ut volutpat orci. Curabitur cursus consequat tortor, sed porta ex eleifend at. Sed iaculis feugiat mauris sed ornare. Etiam sed massa congue est interdum tempus. Sed laoreet aliquam neque vitae feugiat. Cras tortor ligula, lacinia vel elit eget, finibus venenatis tortor.

Fusce ullamcorper feugiat nunc, sit amet aliquet nisi rhoncus eu. Aliquam ut ipsum elit. Fusce a felis quis ex ullamcorper mollis. Interdum et malesuada fames ac ante ipsum primis in faucibus. Fusce a mauris mi. Aliquam auctor convallis est, at efficitur sapien ultricies posuere. Nullam magna ligula, blandit eget lobortis ac, posuere at dui. Fusce sit amet ipsum id sem suscipit vestibulum sit amet sit amet ante. Curabitur pharetra lacus justo, at bibendum magna molestie non. Donec at consequat arcu.

Etiam viverra posuere dui eu ultrices. Praesent nec est ut arcu sagittis vestibulum eu ut ante. Nullam dolor augue, consequat at nunc at, hendrerit blandit felis. Phasellus nulla metus, feugiat nec aliquam non, rhoncus id eros. Pellentesque et justo nec augue sodales rhoncus. Nulla dictum mollis feugiat. Pellentesque placerat viverra pulvinar. Sed pharetra, lorem id tincidunt laoreet, erat dui eleifend sem, nec pulvinar magna velit quis leo.";

fn create(c: &mut Criterion) {
    let words: Vec<_> = TEXT.split_whitespace().collect();

    c.bench_function("create/beef", |b| {
        b.iter(|| {
            let cows: Vec<BeefCow<str>> = words.iter().copied().map(BeefCow::borrowed).collect();
            black_box(cows);
        })
    });

    c.bench_function("create/dairy", |b| {
        b.iter(|| {
            let cows: Vec<DairyCow<str>> = words.iter().copied().map(DairyCow::borrowed).collect();
            black_box(cows);
        })
    });

    c.bench_function("create/std", |b| {
        b.iter(|| {
            let cows: Vec<StdCow<str>> = words.iter().copied().map(StdCow::Borrowed).collect();
            black_box(cows);
        })
    });
}

fn create_mixed(c: &mut Criterion) {
    let words: Vec<_> = TEXT.split_whitespace().collect();

    c.bench_function("create_mixed/beef", |b| {
        b.iter(|| {
            let cows: Vec<BeefCow<str>> = words
                .iter()
                .copied()
                .enumerate()
                .map(|(i, word)| {
                    if i % NTH_WORD == 0 {
                        BeefCow::owned(word.to_owned())
                    } else {
                        BeefCow::borrowed(word)
                    }
                })
                .collect();
            black_box(cows);
        })
    });

    c.bench_function("create_mixed/dairy", |b| {
        b.iter(|| {
            let cows: Vec<DairyCow<str>> = words
                .iter()
                .copied()
                .enumerate()
                .map(|(i, word)| {
                    if i % NTH_WORD == 0 {
                        DairyCow::owned(word.to_owned())
                    } else {
                        DairyCow::borrowed(word)
                    }
                })
                .collect();
            black_box(cows);
        })
    });

    c.bench_function("create_mixed/std", |b| {
        b.iter(|| {
            let cows: Vec<StdCow<str>> = words
                .iter()
                .enumerate()
                .map(|(i, word)| {
                    if i % NTH_WORD == 0 {
                        StdCow::Owned((*word).to_owned())
                    } else {
                        StdCow::Borrowed(*word)
                    }
                })
                .collect();
            black_box(cows);
        })
    });
}

criterion_group!(benches, create, create_mixed);
