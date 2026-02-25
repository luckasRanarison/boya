use std::hint::black_box;

use boya_core::{Gba, bus::BIOS_SIZE};
use criterion::{Criterion, criterion_group, criterion_main};

const BIOS: &[u8; BIOS_SIZE] = include_bytes!("../../../bin/gba_bios.bin");
const ROM: &[u8] = include_bytes!("../../../../../Downloads/bigmap.gba");

fn init_gba() -> Gba {
    let mut gba = Gba::default();

    gba.load_bios(*BIOS);
    gba.load_rom(ROM);
    gba.boot();
    gba.skip_bios();

    for _ in 0..60 {
        gba.step_frame();
    }

    gba
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("first", |b| {
        // let mut gba = init_gba();
        //
        // b.iter(|| {
        //     for _ in 0..60 {
        //         gba.step_frame();
        //     }
        //
        //     black_box(());
        // })
        b.iter_batched(
            init_gba,
            |mut gba| {
                for _ in 0..60 {
                    gba.step_frame();
                }

                black_box(());
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark
}

criterion_main!(benches);
