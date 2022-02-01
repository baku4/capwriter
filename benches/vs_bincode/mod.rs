use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};


// Capwriter

use capwriter::{Saveable, Loadable};

#[inline]
fn save_usize_vec_using_capwriter(vec: &[usize]) -> Vec<u8> {
    let mut buffer = Vec::new();
    vec.save_to(&mut buffer).unwrap();
    buffer
}

#[inline]
fn load_usize_vec_using_capwriter(buffer: &[u8]) -> Vec<usize> {
    let loaded = Vec::<usize>::load_from(buffer).unwrap();
    loaded
}

// Bincode

use bincode::{serialize, deserialize};

#[inline]
fn save_usize_vec_using_bincode(vec: &[usize]) -> Vec<u8> {
    let encoded = serialize(vec).unwrap();
    encoded
}

#[inline]
fn load_usize_vec_using_bincode(buffer: &[u8]) -> Vec<usize> {
    let loaded = deserialize(buffer).unwrap();
    loaded
}


pub fn bench_save_usize_vs_bincode(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("save_usize_vs_bincode");

    group.plot_config(plot_config.clone());

    let vec_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for vec_len in vec_len_list {
        let vec: Vec::<usize> = vec![0; vec_len];
        
        group.bench_with_input(
            BenchmarkId::new("capwriter", vec_len),
            &vec_len,
            |b, _| b.iter(|| {
                save_usize_vec_using_capwriter(black_box(&vec));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("bincode", vec_len),
            &vec_len,
            |b, _| b.iter(|| {
                save_usize_vec_using_bincode(black_box(&vec));
            }
        ));
    }
}

pub fn bench_load_usize_vs_bincode(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("load_usize_vs_bincode");

    group.plot_config(plot_config.clone());

    let vec_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for vec_len in vec_len_list {
        let vec: Vec::<usize> = vec![0; vec_len];

        let buffer_capwriter = save_usize_vec_using_capwriter(&vec);
        let buffer_bincode = save_usize_vec_using_bincode(&vec);
        
        group.bench_with_input(
            BenchmarkId::new("capwriter", vec_len),
            &vec_len,
            |b, _| b.iter(|| {
                load_usize_vec_using_capwriter(black_box(&buffer_capwriter));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("bincode", vec_len),
            &vec_len,
            |b, _| b.iter(|| {
                load_usize_vec_using_bincode(black_box(&buffer_bincode));
            }
        ));
    }
}
