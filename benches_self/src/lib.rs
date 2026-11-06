//! 基准测试共享工具：数据加载、域名反转、确定性伪随机数与内存计量。

use std::fs;
use std::path::PathBuf;

/// `benches_self/majestic_million.txt` 的绝对路径。
pub fn data_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("majestic_million.txt")
}

/// 把 `google.com` 形式的域名按标签逆序并追加结尾点，得到 `com.google.`。
pub fn reverse_domain_bytes(domain: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(domain.len() + 1);
    for label in domain.split(|&b| b == b'.').rev() {
        if label.is_empty() {
            continue;
        }
        out.extend_from_slice(label);
        out.push(b'.');
    }
    out
}

/// 读取全部域名并转成逆序 key。下标 `i` 对应的 value 为 `i + 1`。
pub fn load_keys() -> Vec<Vec<u8>> {
    let raw = fs::read(data_path()).expect("读取 majestic_million.txt 失败");
    let mut keys = Vec::with_capacity(1_000_000);
    for line in raw.split(|&b| b == b'\n') {
        let line = trim_ascii(line);
        if line.is_empty() {
            continue;
        }
        keys.push(reverse_domain_bytes(line));
    }
    keys
}

fn trim_ascii(mut line: &[u8]) -> &[u8] {
    while let [rest @ .., last] = line {
        if *last == b'\r' || *last == b' ' || *last == b'\t' {
            line = rest;
        } else {
            break;
        }
    }
    line
}

/// xorshift64，保证基准测试可复现且不引入额外依赖。
pub struct XorShift64(u64);

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        assert!(seed != 0, "xorshift 种子不能为 0");
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// 返回 `[0, n)` 内的值。
    pub fn below(&mut self, n: usize) -> usize {
        assert!(n > 0);
        (self.next_u64() % n as u64) as usize
    }
}

/// 从 `0..n` 中不重复地抽取 `k` 个下标（部分 Fisher-Yates）。
pub fn sample_indices(n: usize, k: usize, rng: &mut XorShift64) -> Vec<usize> {
    assert!(k <= n);
    let mut idx: Vec<usize> = (0..n).collect();
    for i in 0..k {
        let j = i + rng.below(n - i);
        idx.swap(i, j);
    }
    idx.truncate(k);
    idx
}

/// 生成一条几乎必然不存在的逆序 key：2~4 个标签，标签长度 3~12，字符集 `a-z0-9`。
///
/// 生成结果已经是逆序并带结尾点，可直接用于查询。
pub fn random_miss_key(rng: &mut XorShift64) -> Vec<u8> {
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let label_count = 2 + rng.below(3);
    let mut labels: Vec<Vec<u8>> = Vec::with_capacity(label_count);
    for _ in 0..label_count {
        let len = 3 + rng.below(10);
        let mut label = Vec::with_capacity(len);
        for _ in 0..len {
            label.push(ALPHABET[rng.below(ALPHABET.len())]);
        }
        labels.push(label);
    }

    let mut out = Vec::with_capacity(label_count * 8);
    for label in labels.iter().rev() {
        out.extend_from_slice(label);
        out.push(b'.');
    }
    out
}

/// 从 `/proc/self/status` 读取形如 `VmHWM:  12345 kB` 的字段，单位为 kB。
pub fn status_kb(field: &str) -> u64 {
    let status = fs::read_to_string("/proc/self/status").expect("读取 /proc/self/status 失败");
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix(field) {
            if let Some(rest) = rest.strip_prefix(':') {
                if let Some(value) = rest.split_whitespace().next() {
                    return value.parse().expect("解析 /proc/self/status 数值失败");
                }
            }
        }
    }
    panic!("在 /proc/self/status 中找不到字段 {field}");
}
