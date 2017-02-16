fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    println!("Fourth element: {}", xs[3]);

    // indexing must use the type usize
    let v = vec![1,2,3];
    let i: usize = 0;
    let j: i32 = 0;
    v[i]; // ok
    // v[j]; error

    let mut vs = vec![1, 2, 3, 4, 5];

    for i in &vs { // immutable reference
        println!("A reference to {}", i);
    }

    for i in &mut vs { // mutable reference
        println!("A mutable reference to {}", i);
    }

    for i in vs { // takes the ownership
        println!("Take ownership of the vector and its element {}", i);
    }

    // after that vs cannot be used!

}
