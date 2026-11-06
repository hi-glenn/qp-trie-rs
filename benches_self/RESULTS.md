# fast_radix_trie vs rart：LPM / 插入基准结果

## 环境与口径

| 项 | 值 |
|---|---|
| CPU | Intel Xeon E5-2630 v2（Ivy Bridge，无 AVX2），`taskset -c 6` 单核绑定 |
| 编译 | `RUSTFLAGS="-C target-cpu=native"`，`--release` / `bench` profile |
| 工具链 | rustc 1.99.1，criterion 0.8.2（关闭 plotters/rayon） |
| 被比较库 | `fast_radix_trie 1.2.0`（`RadixMap<u64>`）、`rart 0.11.0`（`AdaptiveRadixTree<VectorKey, u64>`，默认 `simd_keys`） |
| 数据集 | `majestic_million.txt`，1,000,000 条域名，全部唯一、小写 |
| key 形式 | 标签逆序 + 结尾点，如 `google.com` → `com.google.`；累计 key 字节 15,704,346 |
| value | 行号 1..=1,000,000 |
| 统计方式 | 每组跑 3 轮，取中位数；表内为吞吐（次/秒）与折算的单次耗时 |

查询接口：fast_radix 用 `get_longest_common_prefix`（返回匹配前缀切片 + value，基准只取 value），rart 用 `longest_prefix_value_bytes`（只返回值）。两者都返回“最深且带 value 的节点”，热路径均无堆分配。

## LPM 查找吞吐（次/秒）

| 场景 | fast_radix | rart | rart / fast | 单次耗时 fast_radix | 单次耗时 rart |
|---|---:|---:|---:|---:|---:|
| 随机已存在（1024 条/迭代） | 717,580 | 1,039,700 | 1.45× | 1.394 µs | 0.962 µs |
| 随机不存在（1024 条/迭代） | 12,779,000 | 22,678,000 | 1.77× | 78.25 ns | 44.10 ns |
| 单条已存在（`com.google.`） | 4,265,000 | 5,980,000 | 1.40× | 234.45 ns | 167.21 ns |
| 单条不存在 | 22,119,000 | 34,934,000 | 1.58× | 45.21 ns | 28.63 ns |

三轮波动范围（次/秒）：

| 场景 | fast_radix | rart |
|---|---|---|
| 随机已存在 | 702.4 K – 745.9 K | 1.029 M – 1.108 M |
| 随机不存在 | 12.41 M – 13.79 M | 22.33 M – 24.26 M |
| 单条已存在 | 4.20 M – 4.28 M | 5.85 M – 6.05 M |
| 单条不存在 | 21.36 M – 22.31 M | 34.42 M – 35.32 M |

- “随机”组的查询集为 102,400 条（= 100 × 1024），迭代时按 1024 条连续推进窗口，`Throughput::Elements(1024)`。
- “已存在”查询全部为完整存储 key（终点命中，LPM 结果即该 key 本身）；“不存在”查询在计时前已断言两个实现均返回 `None`。
- 随机组单次耗时高于单条组，是 100 万 key（远超 15 MB L3）造成的缓存缺失，属于预期。

## 建树插入吞吐（次/秒）

| 场景 | fast_radix | rart | rart / fast |
|---|---:|---:|---:|
| 全量插入 1,000,000 条 | 838,970 次/s（总 1.192 s） | 1,233,300 次/s（总 0.811 s） | 1.47× |
| 折算单条 | 1191.9 ns | 810.8 ns | — |

三轮原始总耗时：

| 轮次 | fast_radix | rart |
|---|---|---|
| 1 | 1.043 s | 0.818 s |
| 2 | 1.192 s | 0.811 s |
| 3 | 1.200 s | 0.809 s |

fast_radix 后两轮明显偏高且置信区间宽（缓存/分配器状态影响），rart 三轮稳定。因此插入项建议按 0.839 M–0.959 M 次/s 看待。

## 内存占用（建树完成后）

| 指标 | fast_radix | rart | 比值 |
|---|---:|---:|---:|
| VmHWM（峰值） | 62.81 MiB | 185.44 MiB | 2.95× |
| VmRSS（结束时常驻） | 62.81 MiB | 185.44 MiB | 2.95× |
| 累计 key 字节 | 15,704,346 | 15,704,346 | — |

测量方式：独立进程流式逐行读取数据集，反转后立即插入并释放输入行（不驻留全量输入），进程结束前读 `/proc/self/status`。rart 的 key 以 owned `VectorKey` 存入树内，字节计入其内存；fast_radix 则把 key 字节拷贝进节点。

## 结论

1. 查找：rart 在四种场景下快 1.40×–1.77×；已存在随机查询（最接近真实 DNS 命中场景）为 1.45×。
2. 插入：rart 快约 1.47×，但 fast_radix 数据波动更大。
3. 内存：fast_radix 仅为 rart 的约 1/3，是二者最显著的取舍点。
4. 口径差异：插入时 rart 直接移交 owned key（无拷贝），fast_radix 从借用切片拷贝字节，这是各库自然 API 形态，不是逐字节等价的工作量。

## 复现

```bash
cd benches_self
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench lpm
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench insert
cargo run --release --bin mem -- fast_radix
cargo run --release --bin mem -- rart
```
