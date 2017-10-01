fn ref_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = "this is a resource".to_string();
    // 参照1つめ。
    let t = &s;
    // 参照2つめ。同時に2つ存在できる。
    ref_string(&s);
}