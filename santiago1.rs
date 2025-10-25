#[derive(Debug)] // Para poder imprimir con {:?}
struct Circle {
    radius: f64,
    color: String,
    x: f64,
    y: f64,
}

fn main() {
    let my_circle = Circle {
        radius: 5.0,
        color: String::from("red"),
        x: 0.0,
        y: 0.0,
    };
    println!("circulo: {:?}", my_circle); 
}