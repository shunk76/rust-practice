enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停止",
        TrafficLight::Yellow => "注意",
        TrafficLight::Green => "進行",
    }
}

fn main() {
    println!("{}", action(TrafficLight::Red));
    println!("{}", action(TrafficLight::Yellow));
    println!("{}", action(TrafficLight::Green));
}
