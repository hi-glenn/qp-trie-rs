// use qp_trie::Trie;

extern crate qp_trie;

use qp_trie::*;

fn main() {
    // test_get_lpm();

    test_get_lpm2();
}

fn test_get_lpm2() {
    let mut t = Trie::<&[u8], u32>::new();

    println!("abs");
    t.insert("aba".as_bytes(), 1);

    println!("\nabb");
    t.insert("abb".as_bytes(), 2);

    println!("\nabc");
    t.insert("abc".as_bytes(), 3);

    println!("\nabd");
    t.insert("abd".as_bytes(), 4);

    println!("\nabe");
    t.insert("abe".as_bytes(), 5);

    println!("\nabf");
    t.insert("abf".as_bytes(), 6);

    println!("\nabg");
    t.insert("abg".as_bytes(), 7);

    println!("\nabh");
    t.insert("abh".as_bytes(), 8);

    println!("\nabi");
    t.insert("abi".as_bytes(), 9);

    println!("\nabj");
    t.insert("abj".as_bytes(), 10);

    println!("\nabk");
    t.insert("abk".as_bytes(), 11);

    println!("\nabl");
    t.insert("abl".as_bytes(), 12);

    println!("\nabm");
    t.insert("abm".as_bytes(), 13);

    println!("\nabn");
    t.insert("abn".as_bytes(), 14);

    println!("\nabo");
    t.insert("abo".as_bytes(), 15);

    println!("\nab`");
    t.insert("ab`".as_bytes(), 16); // ----

    println!("\nab");
    t.insert("ab".as_bytes(), 17); // ----

    let ret = t.get("abc".as_bytes());
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
