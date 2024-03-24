// use qp_trie::Trie;

extern crate qp_trie;

use qp_trie::*;

fn main() {
    test_get_lpm();
}

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
    println!("ret: {:?}", std::str::from_utf8(ret).expect("Invalid UTF-8"));

    let ret = t.get(k4.as_bytes());
    println!("get: {:?}", ret);
}


