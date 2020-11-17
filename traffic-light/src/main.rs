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
