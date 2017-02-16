// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// activity
fn rect_area(rect : Rectangle) -> f32 {
    let b = rect.p2.x - rect.p1.x;
    let h = rect.p2.y - rect.p1.y;
    b.abs()*h.abs()
}

fn square(p1 : Point, n : f32) -> Rectangle {
    let p2 = Point { x: p1.x + n, y: p1.y + n};
    Rectangle{p1: p1, p2: p2}
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: 0.0, y: 0.0 },
        p2: point,
    };

    println!("rectangle area: {:?}", rect_area(_rectangle));
    let origin = Point {x: 0.0, y: 0.0};
    let square = square(origin, 2.0);
    println!("square coordinates: ({:?}, {:?}),({:?}, {:?})", square.p1.x, square.p1.y, square.p2.x, square.p2.y);

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
