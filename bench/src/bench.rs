use ndcopy::ndshape::{ConstShape2u32, ConstShape3u32};
use ndcopy::{copy2, copy3};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn bench_copy2(c: &mut Criterion) {
    let shape = ConstShape2u32::<100, 100>;
    let src = [1u8; 100 * 100];
    let mut dst = [0u8; 100 * 100];

    let mut group = c.benchmark_group("bench_copy2");
    for &copy_width in [8, 16, 32, 64].iter() {
        let copy_area = copy_width * copy_width;

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("area={}", copy_area)),
            &(),
            |b, _| {
                b.iter(|| {
                    copy2(
                        [copy_width; 2],
                        &src,
                        &shape,
                        [1, 2],
                        &mut dst,
                        &shape,
                        [3, 4],
                    );
                    black_box(&dst);
                });
            },
        );
    }
    group.finish();
}

pub fn bench_copy3(c: &mut Criterion) {
    let shape = ConstShape3u32::<100, 100, 100>;
    let src = [1u8; 100 * 100 * 100];
    let mut dst = [0u8; 100 * 100 * 100];

    let mut group = c.benchmark_group("bench_copy3");
    for &copy_width in [8, 16, 32, 64].iter() {
        let copy_volume = copy_width * copy_width * copy_width;

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("volume={}", copy_volume)),
            &(),
            |b, _| {
                b.iter(|| {
                    copy3(
                        [copy_width; 3],
                        &src,
                        &shape,
                        [1, 2, 3],
                        &mut dst,
                        &shape,
                        [3, 4, 5],
                    );
                    black_box(&dst);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_copy2, bench_copy3);
criterion_main!(benches);
