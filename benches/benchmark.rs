use criterion::{
    criterion_group, criterion_main,
};

mod vs_bincode;
use vs_bincode::{
    bench_save_usize_vs_bincode,
    bench_load_usize_vs_bincode,
};

criterion_group!(
    benches,
    bench_save_usize_vs_bincode,
    bench_load_usize_vs_bincode,
);

criterion_main!(benches);
