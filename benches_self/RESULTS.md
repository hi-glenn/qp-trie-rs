# fast_radix_trie vs rart vs qp-trie：LPM / 插入 / 内存基准结果

## 环境与口径

| 项 | 值 |
|---|---|
| CPU | Intel Xeon E5-2630 v2（Ivy Bridge，无 AVX2），`taskset -c 6` 单核绑定 |
| 编译 | `RUSTFLAGS="-C target-cpu=native"`，rustc 1.99.1，criterion 0.8.2 默认 `bench` profile |
| 被比较实现 | `fast_radix_trie 1.2.0`（`RadixMap<u64>`）、`rart 0.11.0`（`AdaptiveRadixTree<VectorKey, u64>`）、本分支 qp-trie（`Trie<Vec<u8>, u64>` + 新增 `lpm`，path 依赖） |
| 数据集 | `majestic_million.txt` 的 1,000,000 条唯一小写域名，key 为标签逆序加尾点（`google.com` → `com.google.`），累计 15,704,346 字节 |
| value | 行号 1..=1,000,000 |
| 统计 | 每组 3 轮取中位数 |

查询接口分别为 `get_longest_common_prefix`、`longest_prefix_value_bytes`、`lpm`，语义一致：返回最深且带 value 的前缀节点，热路径均无堆分配。

## LPM 查找（次/秒）

| 场景 | fast_radix | rart | qp-trie | 单次耗时 fast / rart / qp |
|---|---:|---:|---:|---:|
| 随机已存在（1024 条/迭代） | 744,024 | 1,149,877 | **1,406,362** | 1344.0 / 869.7 / 711.1 ns |
| 随机不存在（1024 条/迭代） | 14,236,855 | **24,405,939** | 6,888,201 | 70.2 / 41.0 / 145.2 ns |
| 单条已存在（`com.google.`） | 4,318,162 | 5,178,664 | **7,344,301** | 231.6 / 193.1 / 136.2 ns |
| 单条不存在 | 23,837,334 | **36,119,338** | 25,310,048 | 42.0 / 27.7 / 39.5 ns |

- 随机组查询集 102,400 条（100 × 1024），迭代时按 1024 条连续推进窗口，`Throughput::Elements(1024)`；"已存在"为完整存储 key，"不存在"在计时前已断言三方均返回 `None`。
- 随机组单次耗时高于单条组，主因是 100 万 key（远超 15 MB L3）造成的缓存缺失。
- qp-trie 在 miss 上偏慢：沿完整 nybble 路径下潜到叶子后才比对，不提前终止；命中路径则因节点内 key 切片局部性更好而占优。
- 三轮波动均 ≤6%，多数 ≤3%。

## 建树插入

| 场景 | fast_radix | rart | qp-trie |
|---|---:|---:|---:|
| 全量插入 1,000,000 条 | 976,943 次/s（1.0236 s） | **1,214,500 次/s（0.8234 s）** | 949,000 次/s（1.0537 s） |

三者轮间波动均 ≤2%。

## 内存占用（建树完成后）

| 指标 | fast_radix | rart | qp-trie |
|---|---:|---:|---:|
| 峰值 / 结束时常驻（VmHWM / VmRSS） | **62.62 MiB** | 185.44 MiB | 108.75 MiB |

测量方式：独立进程流式逐行读取数据集，反转后立即插入并释放输入行，进程结束前读 `/proc/self/status`；两个字段数值相同，故合并展示。rart 的 `VectorKey` 与 qp-trie 的 `Vec<u8>` 由树接管，fast_radix 把 key 字节拷贝进节点，三者 key 字节均计入。

## 结论

1. 命中查找：qp-trie 最快，随机命中为 fast_radix 的 1.89×、rart 的 1.22×，单条命中为 fast_radix 的 1.70×。
2. 未命中查找：rart 最快，fast_radix 次之，qp-trie 最慢（随机 miss 仅为 fast_radix 的 0.48×）。
3. 插入：rart 最快，fast_radix 与 qp-trie 接近。
4. 内存：fast_radix 最省，qp-trie 居中，rart 最高（约为 fast_radix 的 3×）。
5. 口径差异：插入侧 rart 与 qp-trie 移交 owned key、fast_radix 拷贝借用切片，属各库自然 API 形态，不是逐字节等价的工作量；查询侧三者均无堆分配。

## 复现

```bash
cd benches_self
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench lpm
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench insert
cargo run --release --bin mem -- fast_radix
cargo run --release --bin mem -- rart
cargo run --release --bin mem -- qp_trie
```
