fn main() {
    let rect1 = (30, 50);
    
    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area(rect1)
    );
}

fn area( dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
