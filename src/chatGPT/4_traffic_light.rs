// 問題
// 交通信号（信号機）を表すTrafficLightというenumを作成してください。
// このenumは、Red、Yellow、Greenの3つのバリアントを持つとします。
// TrafficLightには、timeというメソッドを追加してください。
// このメソッドは、各信号の持続時間（秒）を返すものとします。
// 例えば、Redは120秒、Yellowは5秒、Greenは60秒といった具体的な数値を返してください。
// main関数内で、各信号の持続時間を出力する処理を書いてください。

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 120,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 60,
        }
    }
}

fn main() {
    let traffic_light = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];

    for light in traffic_light {
        println!("{:?} {}秒", light, light.time())
    }
}
