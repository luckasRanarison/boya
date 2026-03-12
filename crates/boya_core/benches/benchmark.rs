use std::hint::black_box;

use boya_core::Gba;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

const ROM: &[u8] = include_bytes!(
    "../../../submodules/Emu-Docs/GameBoy Advance/test_roms/tonc_gba_demos/prio_demo.gba"
);

fn init_gba() -> Gba {
    let mut gba = Gba::default();

    gba.boot();
    gba.load_rom(ROM);
    gba.skip_bios();

    gba
}

fn benchmark_fn(c: &mut Criterion) {
    c.bench_function("test 60 frames", |b| {
        b.iter_batched(
            init_gba,
            |mut gba| {
                for _ in 0..60 {
                    gba.step_frame();
                }
                black_box(());
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = benchmark;
    config = Criterion::default().sample_size(10);
    targets = benchmark_fn
}

criterion_main!(benchmark);
