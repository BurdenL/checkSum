fn main() {
    println!("Hello, world!");

    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");


    println!("md5 {:?}", digest);
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}
