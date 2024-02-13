mod container;
use container::Container;

fn main() {
    let mut c = Container::new((1, 2, 3));
    println!("{:?}", c);
    println!("{}", c.0);
    c.0 = 5;
    println!("{:?}", c);
    println!("{}", c.0);
}
