extern crate qp_trie;

use qp_trie::*;

pub(crate) const _RTYPE_BIT_OFS: u32 = 32;
// NS
pub(crate) const _RTYPE_MASK_3: u64 = 0b01 << (_RTYPE_BIT_OFS + 3);

#[test]
fn sub_zone_test() {
    let mut all_zones = Trie::<Vec<u8>, (u64, Trie<Vec<u8>, [u64; 2]>)>::new();

    // 添加 top zone
    all_zones.insert("moc.olleh.".as_bytes().to_vec(), (1, Trie::new()));
    all_zones.insert("moc.elpmaxe.".as_bytes().to_vec(), (11, Trie::new()));

    assert_eq!(all_zones.find_top_zone("moc.".as_bytes()), None, "");

    assert_eq!(all_zones.find_top_zone("moc.xxx.".as_bytes()), None, "");

    assert_eq!(
        all_zones.find_top_zone("moc.olleh.".as_bytes()),
        Some((&(1, Trie::<Vec<u8>, [u64; 2]>::new()), 10)),
        ""
    );

    let (_node_inx, top_zone) = all_zones.get_mut("moc.elpmaxe.".as_bytes()).unwrap();

    // let sub_zone = top_zone.find_top_zone("moc.".as_bytes()).as_mut().unwrap();

    // top_zone.insert("moc.elpmaxe.".as_bytes().to_vec(), [0, 0]);

    top_zone.insert("moc.elpmaxe.www.".as_bytes().to_vec(), [1, 2]);

    top_zone.insert("moc.elpmaxe.ali.".as_bytes().to_vec(), [0x800000000, 3]);
    top_zone.insert("moc.elpmaxe.ali.7".as_bytes().to_vec(), [4, 4]);
    top_zone.insert("moc.elpmaxe.ali.8".as_bytes().to_vec(), [5, 5]);

    top_zone.insert("moc.elpmaxe.x.".as_bytes().to_vec(), [1, 1]);
    top_zone.insert("moc.elpmaxe.x.1.".as_bytes().to_vec(), [2, 0]);
    top_zone.insert("moc.elpmaxe.x.1.2.".as_bytes().to_vec(), [3, 0]);
    top_zone.insert("moc.elpmaxe.x.1.2.3.".as_bytes().to_vec(), [4, 4]);

    let ret = top_zone.find_sub_zone("moc.elpmaxe.ali.8.9.".as_bytes(), _RTYPE_MASK_3);
    println!("ret: {:?}", ret);

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.ali.8.9.".as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[34359738368, 3], 16))),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.x.1.2.3.".as_bytes(), _RTYPE_MASK_3),
        (Some((&[1, 1], 14)), Some((&[4, 4], 20)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.x.1.2.3.4.".as_bytes(), _RTYPE_MASK_3),
        (Some((&[1, 1], 14)), Some((&[4, 4], 20)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.x.1.xdfdfsdfsd.".as_bytes(), _RTYPE_MASK_3),
        (Some((&[1, 1], 14)), Some((&[2, 0], 16)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.x.hello.".as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[1, 1], 14)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone("moc.elpmaxe.hello".as_bytes(), _RTYPE_MASK_3),
        (None, None, None),
        ""
    );

    let (_node_inx, top_zone_hello_com) = all_zones.get_mut("moc.olleh.".as_bytes()).unwrap();

    top_zone_hello_com.insert("moc.olleh.www.".as_bytes().to_vec(), [3, 3]);

    assert_eq!(
        top_zone_hello_com.find_sub_zone("moc.olleh.bbs.".as_bytes(), _RTYPE_MASK_3),
        (None, None, None),
        ""
    );
}

/// save: *.1.a.com
/// find: 2.3.1.a.com
#[test]
fn sub_zone_test_1() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [0, 1]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.3.2.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[0, 1], 8)), None),
        ""
    );
}

/// save: *.1.a.com; 2.1.a.com
/// find: 7.2.1.a.com
#[test]
fn sub_zone_test_2() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [0, 1]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [1, 0]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.7.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 8)), Some((&[1, 0], 10)), None),
        ""
    );
}

/// NS
/// save: *.1.a.com;  1.a.com; 2.1.a.com; *.2.1.a.com
/// find: 7.2.1.a.com
#[test]
fn sub_zone_test_3() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [0x800000001, 2]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [1, 1]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[34359738369, 2], 8))),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[34359738369, 2], 8))),
        ""
    );
}

/// NS
/// save: *.1.a.com; *.2.1.a.com
/// find: 3.2.1.a.com; 4.2.1.a.com; 2.1.a.com; 1.a.com; 7.1.a.com; 7.2.a.com;
#[test]
fn sub_zone_test_4() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [0, 1]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [0, 2]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 8)), Some((&[0, 2], 10)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.4.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 8)), Some((&[0, 2], 10)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 8)), Some((&[0, 2], 10)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[0, 1], 8)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.7.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[0, 1], 8)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.2.7.").as_bytes(), _RTYPE_MASK_3),
        (None, None, None),
        ""
    );
}

/// save: 2.1.a.com;
/// find: 2.1.a.com; a.com; 3.1.a.com;
#[test]
fn sub_zone_test_5() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [1, 0]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [2, 0]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[2, 0], 10)), None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.").as_bytes(), _RTYPE_MASK_3),
        (None, None, None),
        ""
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.3.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[1, 0], 8)), None),
        ""
    );
}

/// save: *.a.com; 1.a.com; 2.1.a.com; *.3.2.1.a.com;
/// find: 7.3.2.1.a.com; 3.1.a.com; 2.1.a.com; 3.2.1.a.com; 4.2.1.a.com; 1.a.com;
#[test]
fn sub_zone_test_6() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.".as_bytes().to_vec(), [0, 1]);
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [2, 0]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [3, 0]);
    top_zone.insert("moc.a.1.2.3.".as_bytes().to_vec(), [0, 4]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.7").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[0, 4], 12)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.3.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[2, 0], 8)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[3, 0], 10)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[0, 4], 12)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.4.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[3, 0], 10)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.").as_bytes(), _RTYPE_MASK_3),
        (Some((&[0, 1], 6)), Some((&[2, 0], 8)), None),
    );
}

/// save: 1.a.com; 2.1.a.com; 3.2.1.a.com;
/// find: a.com; 7.3.2.1.a.com; 3.1.a.com; 2.1.a.com; 3.2.1.a.com;
#[test]
fn sub_zone_test_7() {
    let mut top_zone = Trie::<Vec<u8>, [u64; 2]>::new();
    top_zone.insert("moc.a.".as_bytes().to_vec(), [0, 1]);
    top_zone.insert("moc.a.1.".as_bytes().to_vec(), [0x800000002, 2]);
    top_zone.insert("moc.a.1.2.".as_bytes().to_vec(), [0x800000003, 3]);
    top_zone.insert("moc.a.1.2.3.".as_bytes().to_vec(), [0, 4]);

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.").as_bytes(), _RTYPE_MASK_3),
        (None, Some((&[0, 1], 6)), None),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.7").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.3.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.3.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.2.4.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );

    assert_eq!(
        top_zone.find_sub_zone(("moc.a.1.").as_bytes(), _RTYPE_MASK_3),
        (None, None, Some((&[0x800000002, 2], 8))),
    );
}


// ----------------------------------------------------------

// fn reverse_vec(s: &str) -> Vec<u8> {
//     // 1. 收集字符到向量
//     let mut chars: Vec<char> = s.chars().collect();

//     // 2. 安全边界检查
//     if chars.is_empty() {
//         return Vec::new();
//     }

//     // 3. 双指针反转字符
//     let (mut left, mut right) = (0, chars.len() - 1);
//     while left < right {
//         chars.swap(left, right);
//         left += 1;
//         right -= 1;
//     }

//     // 4. 构建反转后的字符串
//     let reversed_str: String = chars.into_iter().collect();

//     // 5. 打印调试信息
//     println!("反转后字符串: {:?}", reversed_str);

//     // 6. 转换为字节向量
//     let bytes: Vec<u8> = reversed_str.as_bytes().to_vec();

//     // 7. 最终输出
//     println!("原始输入: {}, 字节数据: {:?}", s, bytes);
//     bytes
// }
