fn main() {
    println!("1 + 2 = {}", 1u + 2u);
    println!("1.0 / 2.0 = {}", 1.0f32 / 2.0f32);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operations
    println!("100 XOR 001 is {:t}", 0b100 ^ 0b001);
}