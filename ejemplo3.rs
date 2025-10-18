trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> String;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn description(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    fn description(&self) -> String {
        format!(
            "Triangle with base {} and height {}",
            self.base, self.height
        )
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle {
            width: 10.0,
            height: 5.0,
        }),
        Box::new(Triangle {
            base: 8.0,
            height: 6.0,
        }),
    ];

    for shape in shapes {
        println!("Description: {}", shape.description());
        println!("Area: {}", shape.area());
    }
}
