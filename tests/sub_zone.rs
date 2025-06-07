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

    top_zone.insert("moc.elpmaxe.".as_bytes().to_vec(), [0, 0]);

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
        (None, Some((&[0, 0], 12)), None),
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
