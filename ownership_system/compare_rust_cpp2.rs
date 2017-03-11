fn main(){
    // compare rust and c++
    let mut data = vec![1, 2];
    // get an internal reference
    let x = &data[0];

    // `push` causes the backing storage of `data` to be reallocated.
    // Dangling pointer! Use after free!
    // (this does not compile in Rust)
    data.push(3);

    println!("{}", x);
}
