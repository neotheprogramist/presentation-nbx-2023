trait Shape {
    fn area(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area<T: Shape>(shape: &T) {
    let area = shape.area();
    println!("Area: {}", area);
}

fn main() {
    let rectangle = Rectangle {
        width: 2.0,
        height: 3.0,
    };
    print_area(&rectangle);

    let circle = Circle { radius: 1.0 };
    print_area(&circle);
}
