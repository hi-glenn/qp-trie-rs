# fast_radix_trie vs rart vs qp-trie：LPM / 插入 / 内存基准结果

## 环境与口径

| 项 | 值 |
|---|---|
| 机器 | 48 核共享主机，CPU 为 Intel Xeon Silver 4214 @ 2.20 GHz（Cascade Lake，AVX2），L3 33 MiB |
| 绑核 | `taskset -c 6` 单核绑定（测时主机 load average ≈ 2.5，48 核，属轻载） |
| 编译 | `RUSTFLAGS="-C target-cpu=native"`，rustc 1.98.1，criterion 0.8.2，`bench` profile |
| 代码版本 | 分支 `milestone-1@dev02`，commit `9f5a855` |
| 被比较实现 | `fast_radix_trie 1.2.0`（`RadixMap<u64>`）、`rart 0.11.0`（`AdaptiveRadixTree<VectorKey, u64>`）、本仓库 qp-trie（`Trie<Vec<u8>, u64>` + `lpm`） |
| 数据集 | `majestic_million.txt` 的 1,000,000 条唯一小写域名，key 为标签逆序加尾点（`google.com` → `com.google.`），累计 15,704,346 字节 |
| value | 行号 1..=1,000,000 |
| 统计 | 每组独立跑 3 轮，取中位数；轮间波动除个别项外 ≤3% |

查询接口分别为 `get_longest_common_prefix`、`longest_prefix_value_bytes`、`lpm`，语义一致：返回最深且带 value 的前缀节点，热路径均无堆分配。

## LPM 查找（次/秒）

| 场景 | fast_radix | rart | qp-trie | 单次耗时 fast / rart / qp |
|---|---:|---:|---:|---:|
| 随机已存在（1024 条/迭代） | 1,224,394 | 1,286,739 | **1,529,384** | 816.7 / 777.2 / 653.9 ns |
| 随机不存在（1024 条/迭代） | 14,786,357 | **22,382,884** | 6,786,213 | 67.6 / 44.7 / 147.4 ns |
| 单条已存在（`com.google.`） | 4,788,295 | 6,074,992 | **6,463,131** | 208.8 / 164.6 / 154.7 ns |
| 单条不存在 | 27,583,492 | **34,209,898** | 25,855,704 | 36.3 / 29.2 / 38.7 ns |

- 随机组查询集 102,400 条（100 × 1024），迭代时按 1024 条连续推进窗口，`Throughput::Elements(1024)`；"已存在"为完整存储 key，"不存在"在计时前已断言三方均返回 `None`。
- 随机组单次耗时显著高于单条组，主因是 100 万 key 超出 L3（33 MiB）带来的缓存缺失。
- qp-trie 在 miss 上偏慢：沿完整 nybble 路径下潜到叶子后才比对，不提前终止；命中路径则因节点内 key 切片局部性更好而占优。
- 轮间波动：随机组 ≤3.0%，单条组 ≤4.2%（fast_radix 单条命中 200.6–209.3 ns）。

## 建树插入

| 场景 | fast_radix | rart | qp-trie |
|---|---:|---:|---:|
| 全量插入 1,000,000 条 | 2,247,286 次/s（0.4450 s） | **2,922,413 次/s（0.3422 s）** | 1,407,237 次/s（0.7106 s） |

三者轮间波动均 ≤3%。三者输入形态不同：rart 与 qp-trie 移动 owned key，fast_radix 从借用切片拷贝字节进节点。

## 内存占用（建树完成后）

| 指标 | fast_radix | rart | qp-trie |
|---|---:|---:|---:|
| 峰值 / 结束时常驻（VmHWM / VmRSS） | **62.62 MiB** | 185.25 MiB | 108.38 MiB |

测量方式：独立进程流式逐行读取数据集，反转后立即插入并释放输入行，进程结束前读 `/proc/self/status`；两个字段数值相同，故合并展示。三轮结果完全一致。rart 的 `VectorKey` 与 qp-trie 的 `Vec<u8>` 由树接管，fast_radix 把 key 字节拷贝进节点，三者 key 字节均计入。

## LTO 对照（内联对等条件）

默认 `bench` profile 不开 LTO 时，三者在热路径上的内联程度并不相同（用 `nm`/`objdump` 核对 `lpm` 二进制）：

- qp-trie：`Trie::lpm` / `Node::lpm` 均为 `#[inline(always)]`，热循环内对 qp_trie 的调用为 0，完全内联。
- rart：`longest_prefix_value_bytes` 及遍历循环已内联，但每个节点仍有一次 `VectorPartial::prefix_length_slice` 调用（经 GOT 间接调用）和一次 `Node::seek_child` 调用。
- fast_radix：`get_longest_common_prefix::<[u8]>` 未内联，每次查询一次跨函数调用。

用 `CARGO_PROFILE_BENCH_LTO=fat` 重新编译并同样跑 3 轮后：

- rart：`prefix_length_slice` 被完全内联（符号消失、全二进制 0 处调用），`seek_child` 仍是调用；
- fast_radix：查询函数在两组随机基准中被内联，两个单条基准中仍是调用；
- qp-trie：保持全内联。

即 LTO 下三者的内联程度仍未完全一致，主要变化是 rart 少掉了每节点一次间接调用。

| 场景 | fast_radix | rart | qp-trie | 相对无 LTO 变化 fast / rart / qp |
|---|---:|---:|---:|---:|
| 随机已存在 | 1,212,691（824.6 ns） | 1,309,601（763.6 ns） | **1,577,920（633.8 ns）** | +1.0% / −1.8% / −3.1% |
| 随机不存在 | 15,344,601（65.2 ns） | **23,057,061（43.4 ns）** | 6,926,383（144.4 ns） | −3.6% / −2.9% / −2.0% |
| 单条已存在 | 4,771,600（209.6 ns） | **8,509,094（117.5 ns）** | 6,121,205（163.4 ns） | +0.4% / **−28.6%** / +5.6% |
| 单条不存在 | 25,046,672（39.9 ns） | **38,868,478（25.7 ns）** | 25,556,844（39.1 ns） | +10.1% / −12.0% / +1.2% |
| 插入 1,000,000 条 | 437.3 ms（2,286,853 次/s） | **325.3 ms（3,074,037 次/s）** | 663.0 ms（1,508,336 次/s） | −1.7% / −4.9% / −6.7% |

- 变化最明显的是 rart 单条命中：164.6 → 117.5 ns（−28.6%），超过 qp-trie 成为单条命中最快；插入三方均小幅转快（rart −4.9%、qp-trie −6.7%）。
- qp-trie 的随机命中优势保持不变（1,577,920 次/s，分别为 fast_radix 的 1.30×、rart 的 1.20×）；单条命中因 LTO 反而慢 5.6%，落后于 rart。
- 排名变化仅出现在"单条命中"一项，其余场景的胜者与无 LTO 版一致；内存与编译方式无关，未因 LTO 重跑。

### rart 单条命中为何提升明显

用一个只调用 `longest_prefix_value_bytes` 的独立小程序（同数据集、同绑核、每个 key 1,000 万次、取 2 轮均值）拆分单次查找耗时，结果与"每节点省一次跨 crate 间接调用"的模型一致——收益随路径上前缀比较的工作量增长：

| 查询 key（逆序） | 无 LTO | LTO | 变化 |
|---|---:|---:|---:|
| `com.google.` | 170.0 ns | 116.5 ns | −31.5% |
| `com.facebook.` | 189.4 ns | 135.4 ns | −28.5% |
| `com.amazon.` | 165.8 ns | 108.0 ns | −34.8% |
| `com.wikipedia.` | 177.4 ns | 128.9 ns | −27.4% |
| `com.youtube.` | 213.5 ns | 167.2 ns | −21.7% |
| `com.163.` | 95.0 ns | 84.7 ns | −10.8% |
| `com.qq.` | 83.8 ns | 75.1 ns | −10.3% |
| `com.qq.mail.` | 101.1 ns | 92.4 ns | −8.7% |
| 不存在的 key | 21.6 ns | 20.5 ns | −5.4% |

- 长前缀、深路径的 key 降幅最大（−22%~−35%），短路径 key 只降 9~13 ns，纯 miss 几乎不变。
- 机制是内联而非算法变化：内联后的比较体仍是 8 字节分块比较 + `tzcnt`，与原先的独立函数一致；差别在于每节点少一次间接调用（含 GOT 载入、call/ret、调用前后对循环状态的重载）。
- 单条命中基准的 key 正是 `com.google.`，属于受影响最大的一类，因此相对降幅达 28.6%；随机组每 key 700 ns 以上由缓存缺失主导，故仅降 1.8%~2.9%。
- 对照实验：`codegen-units=1` 但不开 LTO 时 rart 单条命中仍是 164.5 ns（无变化），说明收益来自跨 crate 内联，而不是代码布局；同为"每次查询一次调用"的 fast_radix 只有 +0.4%，本就全内联的 qp-trie 为 +5.6%，两者差异属布局噪声。

## 结论

1. 命中查找：qp-trie 在随机命中上最快，为 fast_radix 的 1.25×、rart 的 1.19×（LTO 下 1.30× / 1.20×）。单条命中在默认编译下 qp-trie 略优（1.35× fast_radix、1.06× rart），但 rart 在 LTO 后反超（117.5 ns vs 163.4 ns）。
2. 未命中查找：rart 最快，fast_radix 次之，qp-trie 最慢（随机 miss 仅为 rart 的 0.30×、fast_radix 的 0.46×；单条 miss 差距较小，默认编译为 fast_radix 的 0.94×、rart 的 0.76×，LTO 下为 1.02× / 0.66×）。
3. 插入：rart 最快，fast_radix 次之，qp-trie 最慢（默认编译耗时约为 rart 的 2.08×、fast_radix 的 1.60×；LTO 下为 2.04× / 1.52×）。
4. 内存：fast_radix 最省，qp-trie 居中（约为 fast_radix 的 1.73×），rart 最高（约为 fast_radix 的 2.96×）。
5. 口径差异：插入侧 rart 与 qp-trie 移交 owned key、fast_radix 拷贝借用切片，属各库自然 API 形态，不是逐字节等价的工作量；查询侧三者均无堆分配。内联条件差异已用 LTO 对照单独量化。

## 复现

```bash
cd benches_self
# 数据集：majestic_million.csv 取第 3 列域名、转小写去重，得到 majestic_million.txt
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench lpm
RUSTFLAGS="-C target-cpu=native" taskset -c 6 cargo bench --bench insert
# LTO 对照（每轮命令都要带该变量，否则会重新以非 LTO 编译）
RUSTFLAGS="-C target-cpu=native" CARGO_PROFILE_BENCH_LTO=fat taskset -c 6 cargo bench --bench lpm
RUSTFLAGS="-C target-cpu=native" CARGO_PROFILE_BENCH_LTO=fat taskset -c 6 cargo bench --bench insert
cargo run --release --bin mem -- fast_radix
cargo run --release --bin mem -- rart
cargo run --release --bin mem -- qp_trie
```
