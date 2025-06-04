// use qp_trie::Trie;

extern crate qp_trie;

use qp_trie::*;

// use qp_trie::Trie as QP_Trie;

fn main() {
    // test_get_lpm();

    // test_get_lpm2();

    // test_get_lpm3();

    // test_zone_lpm4();

    // test_zone_lpm();

    test_top_zone();

    test_get();

    test_find_top_zone_mini();
}

fn test_top_zone() {
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
        top_zone.find_top_zone("1abllll".as_bytes()),
        Some((&(121, Trie::<Vec<u8>, [u64; 2]>::new()), 5)),
        ""
    );

    assert_eq!(
        top_zone.find_top_zone("moc.elpmaxe.ww.".as_bytes()),
        Some((&(11, Trie::<Vec<u8>, [u64; 2]>::new()), 12)),
        ""
    );
}

// 左 4 不同，右 4 相同
fn test_get() {
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

// 左 4 不同，右 4 相同
fn test_find_top_zone_mini() {
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

fn test_zone_lpm() {
    // moc.elpmaxe.www.bus
    // moc.elpmaxe.www
    // moc.elpmaxe
    // moc
    println!("hello test_zone_lpm");

    let mut t = Trie::<&[u8], u64>::new();

    println!("\n moc.");
    t.insert("moc.".as_bytes(), 1);

    println!("\n moc.elpmaxe.");
    t.insert("moc.elpmaxe.".as_bytes(), 11);

    println!("\n moc.elpmaxe.www.");
    t.insert("moc.elpmaxe.www.".as_bytes(), 12);

    println!("\n moc.elpmaxe.www.bus.");
    t.insert("moc.elpmaxe.www.bus.".as_bytes(), 4);

    println!("\n get_lpm mo");

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www.bus123.".as_bytes(), 8);
    println!("🌹1 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www.bus.".as_bytes(), 8);
    println!("🌹2 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www.bus1.".as_bytes(), 8);
    println!("🌹3 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www.bu.".as_bytes(), 8);
    println!("🌹4 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www789.".as_bytes(), 8);
    println!("🌹5 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.www.".as_bytes(), 1);
    println!("🌹6 get_lpm: {:?}\n", ret);

    println!("\n ----------------------");

    let ret = t.lpm_with_mask("moc.elpmaxe.ww.".as_bytes(), 1);
    println!("🌹7 get_lpm: {:?}\n", ret);

    assert_eq!(ret, (Some(&11), Some((&11, 12))), "-----");
}

fn test_zone_lpm4() {
    // moc.elpmaxe.www.bus
    // moc.elpmaxe.www
    // moc.elpmaxe
    // moc
    println!("hello test_zone_lpm");

    let mut t = Trie::<&[u8], u64>::new();

    println!("\n moc");
    t.insert("moc.".as_bytes(), 1);

    let ret = t.lpm_with_mask("moc.elpmaxe.ww".as_bytes(), 1);
    println!("🌹 get_lpm: {:?}\n", ret);
}



// 左 4 相同，右 4 不同
fn test_get_lpm2() {
    let mut t = Trie::<&[u8], u32>::new();

    println!("\naba");
    t.insert("1abaddddd".as_bytes(), 1);

    println!("\nabb");
    t.insert("1abb".as_bytes(), 2);

    println!("\nabc");
    t.insert("1abc".as_bytes(), 3);

    println!("\nabd");
    t.insert("1abd".as_bytes(), 4);

    println!("\nabe");
    t.insert("1abe".as_bytes(), 5);

    println!("\nabf");
    t.insert("1abf".as_bytes(), 6);

    println!("\nabg");
    t.insert("1abg".as_bytes(), 7);

    println!("\nabh");
    t.insert("1abh".as_bytes(), 8);

    println!("\nabi");
    t.insert("1abi".as_bytes(), 9);

    println!("\nabj");
    t.insert("1abj".as_bytes(), 10);

    println!("\nabk");
    t.insert("1abk".as_bytes(), 11);

    println!("\nabl");
    t.insert("1abl".as_bytes(), 12);

    println!("\nabm");
    t.insert("1abm".as_bytes(), 13);

    println!("\nabn");
    t.insert("1abn".as_bytes(), 14);

    println!("\nabo");
    t.insert("1abo".as_bytes(), 15);

    println!("\nab`");
    t.insert("1ab`".as_bytes(), 16); // ----

    println!("\nabq");
    t.insert("1abq".as_bytes(), 18); // ----

    println!("\n1ab");
    t.insert("1ab".as_bytes(), 17); // ----

    println!("\n1ab");
    let ret = t.get("1ab".as_bytes());
    println!("🍟 get: {:?}", ret);

    println!("\n1a");
    t.insert("1a".as_bytes(), 19); // ----

    println!("\n--------------");
    let ret = t.get("1ab`".as_bytes());
    println!("🍟 get: {:?}", ret);

    println!("\n");
    let ret = t.get("1a".as_bytes());
    println!("🍟 get: {:?}", ret);
}

#[allow(dead_code)]
fn test_get_lpm() {
    println!("hello test_get_lpm");

    let mut t = Trie::<&[u8], u32>::new();

    println!("\n abcdefg");
    t.insert("abcdefg".as_bytes(), 1);

    println!("\n abcd");
    t.insert("abcd".as_bytes(), 2);

    println!("\n abce");
    t.insert("abce".as_bytes(), 3);

    println!("\n abcf");
    t.insert("abcf".as_bytes(), 4);

    println!("\n ab");
    t.insert("ab".as_bytes(), 5);

    println!("\n ab`");
    t.insert("ab`".as_bytes(), 6);

    println!("\n get_lpm cbcef");
    let ret = t.lpm("cbcef".as_bytes());
    println!("🌹 get_lpm: {:?}", ret);

    // println!("\n get_lpm ab");
    // let ret = t.get_lpm2("ab".as_bytes()); // hit abcd
    // println!("🌹 get_lpm: {:?}", ret);

    println!("\n get_lpm abc");
    let ret = t.lpm("abc".as_bytes()); // hit abcd
    println!("🌹 get_lpm: {:?}", ret);

    println!("\n get_lpm ab");
    let ret = t.lpm("ab".as_bytes()); // hit abcd
    println!("🌹 get_lpm: {:?}", ret);

    println!("\n longest_common abc");
    let ret = t.longest_common_prefix("abc".as_bytes()); // hit abcd
    println!(
        "🌹 common key: {:?}",
        std::str::from_utf8(ret).expect("Invalid UTF-8")
    );

    // println!("\n longest_common cbcef");
    // let ret = t.longest_common_prefix("cbcef".as_bytes());
    // println!(
    //     "🌹 common key: {:?}",
    //     std::str::from_utf8(ret).expect("Invalid UTF-8")
    // );

    // println!("\n moc.udia");
    // let ret = t.get("moc.udia".as_bytes());
    // println!("get: {:?}", ret);
}
