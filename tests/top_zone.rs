extern crate qp_trie;

use qp_trie::*;

#[test]
fn find_top_zone_mini() {
    // 左 4 不同，右 4 相同

    let mut t = Trie::<&[u8], u64>::new();

    println!("\n1b"); // 0x62
    t.insert("1b".as_bytes(), 5);

    println!("\n1A"); // 0x41
    t.insert("1A".as_bytes(), 1);

    println!("\n1Q"); // 0x51
    t.insert("1Q".as_bytes(), 2);

    println!("\n1a"); // 0x61
    t.insert("1a".as_bytes(), 3);

    println!("\n1q"); // 0x71
    t.insert("1q".as_bytes(), 4);

    assert_eq!(t.find_top_zone("1b".as_bytes()), Some((&(5), 2)), "1b");
    assert_eq!(t.find_top_zone("1A".as_bytes()), Some((&(1), 2)), "1A");
    assert_eq!(t.find_top_zone("1Q".as_bytes()), Some((&(2), 2)), "1Q");
    assert_eq!(t.find_top_zone("1a".as_bytes()), Some((&(3), 2)), "1a");
    assert_eq!(t.find_top_zone("1q".as_bytes()), Some((&(4), 2)), "1q");
}


// 左 4 不同，右 4 相同
#[test]
fn get_zone_mini() {
    let mut t = Trie::<&[u8], u32>::new();

    println!("\n1b"); // 0x62
    t.insert("1b".as_bytes(), 5);

    println!("\n1A"); // 0x41
    t.insert("1A".as_bytes(), 1);

    println!("\n1Q"); // 0x51
    t.insert("1Q".as_bytes(), 2);

    println!("\n1a"); // 0x61
    t.insert("1a".as_bytes(), 3);

    println!("\n1q"); // 0x71
    t.insert("1q".as_bytes(), 4);

    assert_eq!(t.get("1b".as_bytes()), Some(&5), "1b");
    assert_eq!(t.get("1A".as_bytes()), Some(&1), "1A");
    assert_eq!(t.get("1Q".as_bytes()), Some(&2), "1Q");
    assert_eq!(t.get("1a".as_bytes()), Some(&3), "1a");
    assert_eq!(t.get("1q".as_bytes()), Some(&4), "1q");
}