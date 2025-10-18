struct Circle {
    radius: f64,
}

impl Circle {
    // Constructor-like associated function
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method to calculate area
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // Method to change the radius
    fn set_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }
}

fn main() {
    let mut my_circle = Circle::new(5.0);
    println!("Initial area: {}", my_circle.area());

    my_circle.set_radius(7.0);
    println!("New area: {}", my_circle.area());
}
