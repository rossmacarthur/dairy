mod create;
mod maybe;

use criterion::criterion_main;

criterion_main!(create::benches, maybe::benches);
