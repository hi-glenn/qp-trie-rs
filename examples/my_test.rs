// use qp_trie::Trie;

extern crate qp_trie;

use qp_trie::*;

fn main() {
    // test_get_lpm();

    test_get_lpm2();

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

    let k1 = "moc.udiab";
    let k2 = "moc.udiab.www";
    let k2_1 = "moc.udiab.wwx";
    let k2_2 = "moc.udiab.wwy";

    t.insert(k1.as_bytes(), 1);

    t.insert(k2.as_bytes(), 2);

    t.insert(k2_1.as_bytes(), 3);

    t.insert(k2_2.as_bytes(), 4);

    let k3 = "moc.udiab.www.1";
    let ret = t.get_lpm(k3.as_bytes());
    println!("ret: {:?}", ret);

    let k4 = "moc.udiab.w";
    let ret = t.get_lpm(k4.as_bytes());
    println!("ret: {:?}", ret);

    let k5 = "moc.udia";
    let ret = t.longest_common_prefix(k5.as_bytes());
    println!(
        "ret: {:?}",
        std::str::from_utf8(ret).expect("Invalid UTF-8")
    );

    let ret = t.get(k4.as_bytes());
    println!("get: {:?}", ret);
}
