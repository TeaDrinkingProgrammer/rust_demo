// Define the Shape enum with different geometric shapes
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// Implement a method to calculate the area of each shape
impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Area of the circle with radius 5: {}", circle.area());
    println!("Area of the rectangle with width 3 and height 4: {}", rectangle.area());
    println!("Area of the triangle with sides 3, 4, and 5: {}", triangle.area());
}