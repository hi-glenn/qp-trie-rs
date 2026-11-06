//! 最长前缀匹配（LPM）基准测试：fast_radix_trie vs rart vs qp-trie。
//!
//! 查询 key 全部为逆序域名（`com.google.` 形式）。
//! 随机组每次迭代连续查询 [`CHUNK`] 条，窗口在查询集内轮转。

use std::hint::black_box;

use benches_self::{load_keys, random_miss_key, sample_indices, XorShift64};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, Throughput};
use fast_radix_trie::RadixMap;
use qp_trie::Trie;
use rart::{AdaptiveRadixTree, VectorKey};

/// 每次迭代连续查询的 key 数量。
const CHUNK: usize = 1024;
/// 随机查询集大小，取 CHUNK 的整数倍，保证窗口轮转时总是完整的块。
const RANDOM_QUERIES: usize = CHUNK * 100;

type FastMap = RadixMap<u64>;
type RartTree = AdaptiveRadixTree<VectorKey, u64>;
type QpTree = Trie<Vec<u8>, u64>;

fn build_fast(keys: &[Vec<u8>]) -> FastMap {
    let mut map = FastMap::new();
    for (i, key) in keys.iter().enumerate() {
        map.insert(&key[..], i as u64 + 1);
    }
    map
}

fn build_rart(keys: &[Vec<u8>]) -> RartTree {
    let mut tree = RartTree::new();
    for (i, key) in keys.iter().enumerate() {
        tree.insert(VectorKey::new_from_vec(key.clone()), i as u64 + 1);
    }
    tree
}

fn build_qp(keys: &[Vec<u8>]) -> QpTree {
    let mut tree = QpTree::new();
    for (i, key) in keys.iter().enumerate() {
        tree.insert(key.clone(), i as u64 + 1);
    }
    tree
}

/// 在同一分组下依次跑三个实现的基准。
fn bench_three<F, R, Q>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    mut fast: F,
    mut rart: R,
    mut qp: Q,
) where
    F: FnMut() -> u64,
    R: FnMut() -> u64,
    Q: FnMut() -> u64,
{
    group.bench_function("fast_radix", |b| b.iter(|| fast()));
    group.bench_function("rart", |b| b.iter(|| rart()));
    group.bench_function("qp_trie", |b| b.iter(|| qp()));
}

fn bench_lpm(c: &mut Criterion) {
    let keys = load_keys();
    let fast = build_fast(&keys);
    let rart = build_rart(&keys);
    let qp = build_qp(&keys);

    // 先做正确性抽查，避免基准测的是错误行为。
    for key in keys.iter().step_by(997) {
        assert!(
            fast.get_longest_common_prefix(key.as_slice()).is_some(),
            "fast_radix 未命中已存 key"
        );
        assert!(
            rart.longest_prefix_value_bytes(key).is_some(),
            "rart 未命中已存 key"
        );
        assert!(qp.lpm(key.as_slice()).is_some(), "qp-trie 未命中已存 key");
    }

    let mut rng = XorShift64::new(0x2545_F491_4F6C_DD1D);

    // 已存在的随机 key：从全部 key 中无放回抽样。
    let existing: Vec<Vec<u8>> = sample_indices(keys.len(), RANDOM_QUERIES, &mut rng)
        .into_iter()
        .map(|i| keys[i].clone())
        .collect();

    // 不存在的随机 key：生成后立即在三个实现上确认确实全部 miss。
    let missing: Vec<Vec<u8>> = (0..RANDOM_QUERIES)
        .map(|_| random_miss_key(&mut rng))
        .collect();
    for key in &missing {
        assert!(
            fast.get_longest_common_prefix(key.as_slice()).is_none(),
            "fast_radix 意外命中 miss key"
        );
        assert!(
            rart.longest_prefix_value_bytes(key).is_none(),
            "rart 意外命中 miss key"
        );
        assert!(qp.lpm(key.as_slice()).is_none(), "qp-trie 意外命中 miss key");
    }

    // 1. 随机查找已存在域名。
    let mut group = c.benchmark_group("lpm_random_existing");
    group.throughput(Throughput::Elements(CHUNK as u64));
    let mut fast_offset = 0usize;
    let mut rart_offset = 0usize;
    let mut qp_offset = 0usize;
    bench_three(
        &mut group,
        || {
            let start = fast_offset;
            fast_offset = (fast_offset + CHUNK) % existing.len();
            let mut acc = 0u64;
            for key in &existing[start..start + CHUNK] {
                let hit = fast.get_longest_common_prefix(black_box(key.as_slice()));
                acc = acc.wrapping_add(hit.map_or(0, |(_, value)| *value));
            }
            acc
        },
        || {
            let start = rart_offset;
            rart_offset = (rart_offset + CHUNK) % existing.len();
            let mut acc = 0u64;
            for key in &existing[start..start + CHUNK] {
                let hit = rart.longest_prefix_value_bytes(black_box(key.as_slice()));
                acc = acc.wrapping_add(hit.copied().unwrap_or(0));
            }
            acc
        },
        || {
            let start = qp_offset;
            qp_offset = (qp_offset + CHUNK) % existing.len();
            let mut acc = 0u64;
            for key in &existing[start..start + CHUNK] {
                let hit = qp.lpm(black_box(key.as_slice()));
                acc = acc.wrapping_add(hit.copied().unwrap_or(0));
            }
            acc
        },
    );
    group.finish();

    // 2. 随机查找不存在的域名。
    let mut group = c.benchmark_group("lpm_random_missing");
    group.throughput(Throughput::Elements(CHUNK as u64));
    let mut fast_offset = 0usize;
    let mut rart_offset = 0usize;
    let mut qp_offset = 0usize;
    bench_three(
        &mut group,
        || {
            let start = fast_offset;
            fast_offset = (fast_offset + CHUNK) % missing.len();
            let mut acc = 0u64;
            for key in &missing[start..start + CHUNK] {
                let hit = fast.get_longest_common_prefix(black_box(key.as_slice()));
                acc = acc.wrapping_add(match hit {
                    Some((_, value)) => *value,
                    None => 1,
                });
            }
            acc
        },
        || {
            let start = rart_offset;
            rart_offset = (rart_offset + CHUNK) % missing.len();
            let mut acc = 0u64;
            for key in &missing[start..start + CHUNK] {
                let hit = rart.longest_prefix_value_bytes(black_box(key.as_slice()));
                acc = acc.wrapping_add(hit.copied().unwrap_or(1));
            }
            acc
        },
        || {
            let start = qp_offset;
            qp_offset = (qp_offset + CHUNK) % missing.len();
            let mut acc = 0u64;
            for key in &missing[start..start + CHUNK] {
                let hit = qp.lpm(black_box(key.as_slice()));
                acc = acc.wrapping_add(hit.copied().unwrap_or(1));
            }
            acc
        },
    );
    group.finish();

    // 3. 单条已存在域名。
    let single_existing = b"com.google.".to_vec();
    assert!(fast
        .get_longest_common_prefix(single_existing.as_slice())
        .is_some());
    assert!(rart
        .longest_prefix_value_bytes(single_existing.as_slice())
        .is_some());
    assert!(qp.lpm(single_existing.as_slice()).is_some());

    let mut group = c.benchmark_group("lpm_single_existing");
    bench_three(
        &mut group,
        || {
            let hit = fast.get_longest_common_prefix(black_box(single_existing.as_slice()));
            hit.map_or(0, |(_, value)| *value)
        },
        || {
            let hit = rart.longest_prefix_value_bytes(black_box(single_existing.as_slice()));
            hit.copied().unwrap_or(0)
        },
        || {
            let hit = qp.lpm(black_box(single_existing.as_slice()));
            hit.copied().unwrap_or(0)
        },
    );
    group.finish();

    // 4. 单条不存在域名。
    let single_missing = random_miss_key(&mut XorShift64::new(0x1234_5678_9ABC_DEF1));
    assert!(fast
        .get_longest_common_prefix(single_missing.as_slice())
        .is_none());
    assert!(rart
        .longest_prefix_value_bytes(single_missing.as_slice())
        .is_none());
    assert!(qp.lpm(single_missing.as_slice()).is_none());

    let mut group = c.benchmark_group("lpm_single_missing");
    bench_three(
        &mut group,
        || {
            let hit = fast.get_longest_common_prefix(black_box(single_missing.as_slice()));
            match hit {
                Some((_, value)) => *value,
                None => 1,
            }
        },
        || {
            let hit = rart.longest_prefix_value_bytes(black_box(single_missing.as_slice()));
            hit.copied().unwrap_or(1)
        },
        || {
            let hit = qp.lpm(black_box(single_missing.as_slice()));
            hit.copied().unwrap_or(1)
        },
    );
    group.finish();
}

criterion_group!(benches, bench_lpm);
criterion_main!(benches);
