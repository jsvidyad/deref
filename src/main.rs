mod array;
mod container;
mod vector;

use array::Array;
use container::Container;
use vector::Vector;

fn main() {
    let mut c = Container::new((1, 2, 3));
    println!("{:?}", c);
    println!("{}", c.0);
    c.0 = 5;
    println!("{:?}", c);
    println!("{}", c.0);
    vector_test();
    array_test();
}

fn vector_test() {
    let mut v = Vector::new(vec![1, 2, 3]);
    println!("{:?}", v);
    println!("{}", &v[0]);
    let temp = &mut v;
    temp[0] = 7;
    println!("{:?}", v);
    println!("{}", &v[0]);
    println!("{}", v[0]);
}

fn array_test() {
    let mut a = Array::new([1, 2, 3, 4, 5]);
    println!("{:?}", a);
    println!("{}", &a[0]);
    let temp = &mut a;
    temp[0] = 11;
    println!("{:?}", a);
    println!("{}", &a[0]);
    println!("{}", a[0]);
}
