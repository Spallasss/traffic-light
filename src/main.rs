enum TrafficLight {
	Red,
	Green,
	Yellow,
}

pub trait Duration {
    fn time_length(&self) -> u8;
}

impl Duration for TrafficLight {
    fn time_length(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 10,
        }
    }
}

fn main() {
    let r = TrafficLight::Red;
    let g = TrafficLight::Green;
    let y = TrafficLight::Yellow;
    println!("red duration is {:?}",r.time_length());
    println!("green duration is {:?}",g.time_length());
    println!("yellow duration is {:?}",y.time_length());
}
