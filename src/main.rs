mod helloworld;

fn do_something<'a>(x: &'a mut (u64,  u64)) -> &'a mut u64 {
    &mut x.0
}

fn main() {
    let mut x = (7, 8);
    let y = do_something(&mut x);
    println!("{:?}", y);
    println!("{:?}", x);

    println!("Hello, world!");
}
