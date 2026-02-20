use rand::RngExt;

fn main() {
    // create a random number generator
    let mut rng = rand::rng();

    let n1: u8 = rng.random::<u8>();
    let n2: u16 = rng.random::<u16>();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.random::<u32>());
    println!("Random i32: {}", rng.random::<i32>());

    // random within range
    println!("Integer[0-10]: {}", rng.random_range(0..10));    // not including 10
    println!("Float: {}", rng.random_range(0.0..10.0));

}
