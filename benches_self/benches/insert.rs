//! 全量插入基准：把 100 万条逆序域名插入空树。
//!
//! 计时只包含 insert，key 的准备工作放在 `iter_batched` 的 setup 中。
//! 注意三者输入形态不同：rart 与 qp-trie 移动 owned key（qp-trie 的
//! `Trie<Vec<u8>, _>` 直接接管 `Vec<u8>`，rart 接管 `VectorKey`），
//! fast_radix 从借用切片拷贝字节进节点。

use std::time::Duration;

use benches_self::load_keys;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use fast_radix_trie::RadixMap;
use qp_trie::Trie;
use rart::{AdaptiveRadixTree, VectorKey};

fn bench_insert(c: &mut Criterion) {
    let keys = load_keys();
    let entries = keys.len() as u64;

    let mut group = c.benchmark_group("insert_all");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));
    group.throughput(Throughput::Elements(entries));

    group.bench_function("fast_radix", |b| {
        b.iter_batched(
            || keys.clone(),
            |keys| {
                let mut map = RadixMap::new();
                for (i, key) in keys.iter().enumerate() {
                    map.insert(&key[..], i as u64 + 1);
                }
                map
            },
            BatchSize::PerIteration,
        )
    });

    group.bench_function("rart", |b| {
        b.iter_batched(
            || {
                keys.iter()
                    .map(|key| VectorKey::new_from_vec(key.clone()))
                    .collect::<Vec<_>>()
            },
            |vec_keys| {
                let mut tree = AdaptiveRadixTree::<VectorKey, u64>::new();
                for (i, key) in vec_keys.into_iter().enumerate() {
                    tree.insert(key, i as u64 + 1);
                }
                tree
            },
            BatchSize::PerIteration,
        )
    });

    group.bench_function("qp_trie", |b| {
        b.iter_batched(
            || keys.clone(),
            |keys| {
                let mut tree = Trie::<Vec<u8>, u64>::new();
                for (i, key) in keys.into_iter().enumerate() {
                    tree.insert(key, i as u64 + 1);
                }
                tree
            },
            BatchSize::PerIteration,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
