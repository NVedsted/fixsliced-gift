use criterion::{Criterion, criterion_group, criterion_main};

use fixsliced_gift::gift128::{Block, encrypt_masked, Key, mask_block};
use fixsliced_gift::gift128::key_schedule::mask_key;

const KEY: Key = [
    0xd0, 0xf5, 0xc5, 0x9a, 0x77, 0x00, 0xd3, 0xe7,
    0x99, 0x02, 0x8f, 0xa9, 0xf9, 0x0a, 0xd8, 0x37,
];

const KEY_MASKS: Block = [
    0x1d, 0x54, 0xf0, 0xae, 0x54, 0x0a, 0xaf, 0x8c,
    0xb3, 0xd7, 0x7d, 0x46, 0x4a, 0xac, 0xa1, 0xb4
];

const PLAINTEXT: Block = [
    0xe3, 0x9c, 0x14, 0x1f, 0xa5, 0x7d, 0xba, 0x43,
    0xf0, 0x8a, 0x85, 0xb6, 0xa9, 0x1f, 0x86, 0xc1,
];

const PLAINTEXT_MASKS: Block = [
    0x1d, 0x54, 0xf0, 0x8e, 0x55, 0x0a, 0xaf, 0x8c,
    0xb3, 0xd2, 0x7d, 0x46, 0x4a, 0xaf, 0xa1, 0xb4
];


fn masked_benchmark(c: &mut Criterion) {
    let masked_key = mask_key(&KEY, &KEY_MASKS);
    let masked_plaintext = mask_block(&PLAINTEXT, &PLAINTEXT_MASKS);

    c.bench_function("uhm", |b| b.iter(|| {
        let mut masked_ciphertext = [Default::default(); PLAINTEXT.len()];
        encrypt_masked(&masked_plaintext, &masked_key, &mut masked_ciphertext);
    }));
}

criterion_group!(benches, masked_benchmark);
criterion_main!(benches);
