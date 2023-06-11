// T needs to implement PartialOrd and Copy
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point <T> {
    x: T,
    y: T,
}

// Need to specifcy that T is a float
impl Point <f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);

    // println!("The largest number is {}", result);

    // Define a point
    let p1 = Point { x: 5.0, y: 10.0 };

    // Find the distance from the origin
    println!("Distance from origin: {}", p1.distance_from_origin());

}

