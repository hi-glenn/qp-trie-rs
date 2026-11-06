//! 单独进程测量建树后的内存占用。
//!
//! 用法：`mem <fast_radix|rart>`
//!
//! 流式逐行读取数据集，反转后立即插入并释放输入行，不驻留全量输入。
//! 输出峰值（VmHWM）与结束时常驻（VmRSS）内存，以及累计 key 字节数。

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use benches_self::{data_path, reverse_domain_bytes, status_kb};
use fast_radix_trie::RadixMap;
use rart::{AdaptiveRadixTree, VectorKey};

fn report(tree: &str, entries: u64, key_bytes: u64) {
    let hwm_kb = status_kb("VmHWM");
    let rss_kb = status_kb("VmRSS");
    println!("tree={tree}");
    println!("entries={entries}");
    println!("key_bytes_total={key_bytes}");
    println!("VmHWM={:.2} MiB", hwm_kb as f64 / 1024.0);
    println!("VmRSS={:.2} MiB", rss_kb as f64 / 1024.0);
}

fn main() {
    let kind = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("用法: mem <fast_radix|rart>");
        process::exit(2);
    });

    let reader = BufReader::new(File::open(data_path()).expect("打开数据集失败"));
    let mut line = Vec::new();
    let mut line_reader = reader;
    let mut entries = 0u64;
    let mut key_bytes = 0u64;

    macro_rules! fill_line {
        () => {{
            line.clear();
            let read = line_reader.read_until(b'\n', &mut line).expect("读取数据失败");
            read
        }};
    }

    match kind.as_str() {
        "fast_radix" => {
            let mut map = RadixMap::new();
            while fill_line!() > 0 {
                let domain = trim_line(&line);
                if domain.is_empty() {
                    continue;
                }
                let key = reverse_domain_bytes(domain);
                key_bytes += key.len() as u64;
                entries += 1;
                map.insert(&key[..], entries);
                // key 在这里立即释放，只有树内部保留了字节拷贝。
            }
            let len = map.len() as u64;
            report("fast_radix", len, key_bytes);
            std::hint::black_box(&map);
        }
        "rart" => {
            let mut tree = AdaptiveRadixTree::<VectorKey, u64>::new();
            while fill_line!() > 0 {
                let domain = trim_line(&line);
                if domain.is_empty() {
                    continue;
                }
                let key = reverse_domain_bytes(domain);
                key_bytes += key.len() as u64;
                entries += 1;
                tree.insert(VectorKey::new_from_vec(key), entries);
                // key 的所有权被树接管，随树一起计入内存。
            }
            let len = tree.len() as u64;
            report("rart", len, key_bytes);
            std::hint::black_box(&tree);
        }
        other => {
            eprintln!("未知实现: {other}，可选 fast_radix | rart");
            process::exit(2);
        }
    }
}

fn trim_line(line: &[u8]) -> &[u8] {
    let mut line = line;
    while let [rest @ .., last] = line {
        if *last == b'\n' || *last == b'\r' || *last == b' ' || *last == b'\t' {
            line = rest;
        } else {
            break;
        }
    }
    line
}
