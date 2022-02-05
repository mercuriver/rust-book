#[derive(Debug)]
enum IpAddr {
    V4(String), 
    V4_2(u32, u32, u32, u32),
    V6(String)
}


fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let home2 = IpAddr::V4_2(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("home: {:?}", home);
    println!("home2: {:?}", home2);
    println!("loopback: {:?}", loopback);
}
