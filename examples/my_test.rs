// use qp_trie::Trie;

extern crate qp_trie;

use qp_trie::*;

fn main() {
    test_get_lpm();

    // test_get_lpm2();

    // test_get_lpm3();
}

// 左 4 不同，右 4 相同
fn test_get_lpm3() {
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

    println!("\n--------------");
    let ret = t.get("1b".as_bytes());
    println!("🍟 get: {:?}", ret);
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
    let ret = t.get_lpm2("cbcef".as_bytes());
    println!("🌹 get_lpm: {:?}", ret);

    // println!("\n get_lpm ab");
    // let ret = t.get_lpm2("ab".as_bytes()); // hit abcd
    // println!("🌹 get_lpm: {:?}", ret);

    println!("\n get_lpm abc");
    let ret = t.get_lpm2("abc".as_bytes()); // hit abcd
    println!("🌹 get_lpm: {:?}", ret);

    println!("\n get_lpm ab");
    let ret = t.get_lpm2("ab".as_bytes()); // hit abcd
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
