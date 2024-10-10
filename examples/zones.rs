
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use likely_stable::{likely, unlikely};

extern crate qp_trie;
use qp_trie::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn reverse_in_place(s: &mut String) {
    let len = s.len();
    let mid = len / 2;
    unsafe {
        let bytes = s.as_bytes_mut();
        for i in 0..mid {
            bytes.swap(i, len - i - 1);
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameKey {
    pub name_rev: [u8; 256],
    pub name_beg_inx: usize,
}

pub fn gen_name_key(name_key: &mut NameKey, name: &str) {
    // name_key.name_reversed
    // baidu.com 存储为 [0]moc[3]udiab[5]

    // init
    name_key.name_beg_inx = 255;

    if name.len() == 0 {
        name_key.name_rev[255] = 0;
        name_key.name_beg_inx = 255;
    } else {
        let section_lens: Vec<usize> = name.split('.').map(|section| section.len()).collect();

        let mut section_lens_iter = section_lens.iter();

        name_key.name_rev[name_key.name_beg_inx] = *section_lens_iter.next().unwrap() as _;
        name_key.name_beg_inx -= 1;

        for e in name.as_bytes() {
            if  *e == b'.' {
                name_key.name_rev[name_key.name_beg_inx] = *section_lens_iter.next().unwrap() as _;
            } else {
                if  (*e >= b'a' && *e <= b'z')
                        || (*e >= b'0' && *e <= b'9')
                        || *e == b'-'
                        || *e == b'_'
                 {
                    // 因为前方已经有逻辑保证都为小写字母，所以无需判断 (*e >= b'A' && *e <= b'Z')
                    name_key.name_rev[name_key.name_beg_inx] = *e;
                } else {
                    // FIXME: 返回错误
                    println!("invalid: {};", *e);
                }
            }
            name_key.name_beg_inx -= 1;
        }

        name_key.name_rev[name_key.name_beg_inx] = 0;
    }
}



fn main() {
    println!("----");

    perf();
}

fn perf() {
    let mut nm_list: Vec<NameKey> = Vec::with_capacity(2000000);

    let mut trie = Trie::new();

    let mut count = 0_u64;
    if let Ok(lines) = read_lines("./top-1000-domains.txt") {
        for line in lines {
            if let Ok(name) = line {
                let mut nm_k = NameKey {
                    name_rev: [0_u8; 256],
                    name_beg_inx: 255,
                };
                gen_name_key(&mut nm_k, name.as_str());
                nm_list.push(nm_k);
            }

            count += 1;
            if count >= 1001 {
                break;
            }
        }
    }

    // -----

    let mut inx: u32 = 0;

    let set_begin = std::time::SystemTime::now();
    for k in nm_list.iter() {
        // println!("k: {:?}", k);
        trie.insert(k.name_rev[k.name_beg_inx..].to_vec(), inx);
        inx += 1;
    }
    let set_end = std::time::SystemTime::now();

    let get_begin = std::time::SystemTime::now();
    let mut count = 0_u64;
    for k in nm_list.iter() {
        // let v = trie.get(&k.name_rev[k.name_beg_inx..]);
        // println!("--------get: {:?}", v);

        let _v1 = trie.lpm(&k.name_rev[k.name_beg_inx..]);
        let _v2 = trie.lpm_with_mask(&k.name_rev[k.name_beg_inx..], 0);
        println!("😡🍟 -------get_zone_lpm: v1: {:?}; v2: {:?}; k: {:?};", _v1, _v2, std::str::from_utf8(&k.name_rev[k.name_beg_inx..]).unwrap());

        count += 1;

        if count >= 100 {
            break;
        }
    }

    let get_end = std::time::SystemTime::now();

    // ---

    // let get_begin_single = std::time::SystemTime::now();
    // for k in nm_list.iter() {
    //     // let v = trie.get(&k.name_rev[k.name_beg_inx..]);
    //     // println!("--------get: {:?}", v);

    //     let _v2 = trie.lpm(&nm_list[1000].name_rev[nm_list[1000].name_beg_inx..]);
    //     // println!("-------get_zone_lpm: {:?}", v2);
    // }
    // let get_end_single = std::time::SystemTime::now();

    println!(
        "Set all used time: {}ms",
        set_end.duration_since(set_begin).unwrap().as_millis()
    );

    println!(
        "Get all used time: {}ms",
        get_end.duration_since(get_begin).unwrap().as_millis()
    );

    // println!(
    //     "Get single used time: {}ms",
    //     get_end_single
    //         .duration_since(get_begin_single)
    //         .unwrap()
    //         .as_millis()
    // );
}