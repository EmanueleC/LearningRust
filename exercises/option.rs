// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)

/* fn main() {
    let mut list = vec![3];

    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap();
    println!("The second-to-last item in the list is {:?}", second_to_last);
}*/

fn pop_vec_res(opt : Option<i32>) -> () {
    match opt {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot pop an empty vector"),
    }
}

fn main() {
    let mut list = vec![3];

    let last = list.pop();
    pop_vec_res(last);

    let second_to_last = list.pop();
    pop_vec_res(second_to_last);
}
