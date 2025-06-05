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

#[test]
fn find_top_zone() {
    let mut top_zone = Trie::<Vec<u8>, (u64, Trie<Vec<u8>, [u64; 2]>)>::new();

    top_zone.insert("moc.".as_bytes().to_vec(), (1, Trie::new()));
    top_zone.insert("moc.elpmaxe.".as_bytes().to_vec(), (11, Trie::new()));

    println!("\naba");
    top_zone.insert("1abaddddd".as_bytes().to_vec(), (1, Trie::new()));

    println!("\nabb");
    top_zone.insert("1abb".as_bytes().to_vec(), (2, Trie::new()));

    println!("\nabc");
    top_zone.insert("1abc".as_bytes().to_vec(), (3, Trie::new()));

    println!("\nabd");
    top_zone.insert("1abd".as_bytes().to_vec(), (4, Trie::new()));

    println!("\nabe");
    top_zone.insert("1abe".as_bytes().to_vec(), (5, Trie::new()));

    println!("\nabf");
    top_zone.insert("1abf".as_bytes().to_vec(), (6, Trie::new()));

    println!("\nabg");
    top_zone.insert("1abg".as_bytes().to_vec(), (7, Trie::new()));

    println!("\nabh");
    top_zone.insert("1abh".as_bytes().to_vec(), (8, Trie::new()));

    println!("\nabi");
    top_zone.insert("1abi".as_bytes().to_vec(), (9, Trie::new()));

    println!("\nabj");
    top_zone.insert("1abj".as_bytes().to_vec(), (10, Trie::new()));

    println!("\nabk");
    top_zone.insert("1abk".as_bytes().to_vec(), (11, Trie::new()));

    println!("\nabl");
    top_zone.insert("1abl".as_bytes().to_vec(), (12, Trie::new()));

    println!("\nabl");
    top_zone.insert("1abll".as_bytes().to_vec(), (121, Trie::new()));

    println!("\nabm");
    top_zone.insert("1abm".as_bytes().to_vec(), (13, Trie::new()));

    println!("\nabn");
    top_zone.insert("1abn".as_bytes().to_vec(), (14, Trie::new()));

    println!("\nabo");
    top_zone.insert("1abo".as_bytes().to_vec(), (15, Trie::new()));

    println!("\nab`");
    top_zone.insert("1ab`".as_bytes().to_vec(), (16, Trie::new())); // ----

    println!("\nabq");
    top_zone.insert("1abq".as_bytes().to_vec(), (17, Trie::new())); // ----

    println!("\n1ab");
    top_zone.insert("1ab".as_bytes().to_vec(), (18, Trie::new())); // ----

    let ret = top_zone.find_top_zone("1abd".as_bytes());
    println!("🌹7 get_lpm: {:?}\n", ret);

    assert_eq!(
        top_zone.find_top_zone("1abaddddd".as_bytes()),
        Some((&(1, Trie::<Vec<u8>, [u64; 2]>::new()), 9)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abb".as_bytes()),
        Some((&(2, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abc".as_bytes()),
        Some((&(3, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abd".as_bytes()),
        Some((&(4, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abe".as_bytes()),
        Some((&(5, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abf".as_bytes()),
        Some((&(6, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abg".as_bytes()),
        Some((&(7, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abh".as_bytes()),
        Some((&(8, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abi".as_bytes()),
        Some((&(9, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abj".as_bytes()),
        Some((&(10, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abk".as_bytes()),
        Some((&(11, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abl".as_bytes()),
        Some((&(12, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abllll".as_bytes()),
        Some((&(121, Trie::<Vec<u8>, [u64; 2]>::new()), 5)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abm".as_bytes()),
        Some((&(13, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abn".as_bytes()),
        Some((&(14, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abo".as_bytes()),
        Some((&(15, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1ab`".as_bytes()),
        Some((&(16, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abq".as_bytes()),
        Some((&(17, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1ab".as_bytes()),
        Some((&(18, Trie::<Vec<u8>, [u64; 2]>::new()), 3)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abb".as_bytes()),
        Some((&(2, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("1abc".as_bytes()),
        Some((&(3, Trie::<Vec<u8>, [u64; 2]>::new()), 4)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("moc.elpmaxe.ww.".as_bytes()),
        Some((&(11, Trie::<Vec<u8>, [u64; 2]>::new()), 12)),
        ""
    );
}
