#[derive(Debug)]
struct Rectangle {
    w: i32,
    h: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.w * self.h
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.h > rect.h && self.w > rect.w
    }

    fn square(size: i32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            w: 8,
            h: 7,
        };
        let smaller = Rectangle {
            w: 6,
            h: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            w: 8,
            h: 7,
        };
        let smaller = Rectangle {
            w: 6,
            h: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn can_hold_equal() {
        let larger = Rectangle {
            w: 8,
            h: 7,
        };

        assert!(larger.can_hold(&larger));
    }
}

fn main() {
    let w = 30;
    let h = 40;

    println!("Area = {}", area(w, h));

    let rect1 = (40, 50);
    println!("Area2 = {}", area2(rect1));

    let rect2 = Rectangle {
        w: 40,
        h: dbg!(40),
    };
    println!("Area3 = {}", area3(&rect2));

    println!("rect2 is {:#?}", rect2);

    dbg!(&rect2);

    println!("Area = {}", rect2.area());

    let rect3 = Rectangle {
        h: 30,
        w: 30,
    };
    println!("rect2 can hold rect3? {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect2? {}", rect3.can_hold(&rect2));

    let sq = Rectangle::square(4);

    println!("sq area = {}", sq.area());
}

fn area3(rect: &Rectangle) -> i32 {
    rect.h * rect.w
}

fn area2(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn area(w: i32, h: i32) -> i32 {
    w * h
}
