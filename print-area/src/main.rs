pub trait HasArea {
    type Output;
    fn get_area(&self) -> Self::Output;
}

struct Square<T> {
    side: T,
}

struct Circle<T> {
    radius: T,
}

struct Triangle<T> {
    base: T,
    hight: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> HasArea for Square<T> {
    type Output = T;

    fn get_area(&self) -> Self::Output {
        self.side * self.side
    }
}

impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Circle<T> {
    type Output = f64;

    fn get_area(&self) -> Self::Output {
       (self.radius * self.radius).into() * std::f64::consts::PI
    }
}

impl<T: std::ops::Mul<Output = T> + Into<f64> + Copy> HasArea for Triangle<T> {
    type Output = f64;

    fn get_area(&self) -> Self::Output {
        (self.base * self.hight).into() * 0.5
    }
}

fn main() {
    let s = Square{side: 10};
    println!("square: {}", s.get_area());

    let c = Circle{radius:3};
    println!("circle: {}", c.get_area());

    let t = Triangle{base: 3, hight: 5};
    println!("triangle: {}", t.get_area());
}