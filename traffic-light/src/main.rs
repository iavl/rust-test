#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    fn time(&self) -> i32;
}

impl Light for TrafficLight {
    fn time(&self) -> i32 {
        return match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 35,
            TrafficLight::Yellow=> 5,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("traffic light is {:?}, {:?} seconds", red, red.time());
    println!("traffic light is {:?}, {:?} seconds", green, green.time());
    println!("traffic light is {:?}, {:?} seconds", yellow, yellow.time());
}
