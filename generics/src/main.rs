use std::fmt::Display;

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    let number_list = vec![34, 45, 67, 89];

    let mut largest = get_largest(number_list);

    println!("{largest}");
}

fn get_largest<T>(list: &[T]) -> impl PartialOrd + Copy + Display
    where T: PartialOrd + Copy {
    let mut largest = &list[0];

    for el in list {
        if el > largest {
            largest = el;
        }
    }

    largest
}
