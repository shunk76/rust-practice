// 1. Trait名は`Vehicle`とし、`speed`と`fuel`というメソッドを持たせる。
// 2. `Car`と`Airplane`という2つのstructを定義する。
// 3. 上記の2つのstructで`Vehicle` traitを実装する。
// 4. `Transport`というenumを定義し、`Car`と`Airplane`のいずれかを持つようにする。
// 5. `Transport` enumにも`Vehicle` traitを実装させる。
// 6. 各structとenumで`speed`と`fuel`の挙動を違う形で実装する（例: Carは燃料によって速度が変わるなど）。

trait Vehicle {
    fn speed(&self) -> f64;
    fn fuel(&self) -> f64;
}

struct Car {
    speed: f64,
    fuel: f64,
}

struct Airplane {
    speed: f64,
    fuel: f64,
}

enum Transport {
    Car(Car),
    Airplane(Airplane),
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.speed
    }

    fn fuel(&self) -> f64 {
        self.fuel
    }
}

impl Vehicle for Airplane {
    fn speed(&self) -> f64 {
        self.speed
    }

    fn fuel(&self) -> f64 {
        self.fuel
    }
}

impl Vehicle for Transport {
    fn speed(&self) -> f64 {
        match self {
            Transport::Car(car) => car.speed(),
            Transport::Airplane(plane) => plane.speed(),
        }
    }

    fn fuel(&self) -> f64 {
        match self {
            Transport::Car(car) => car.fuel(),
            Transport::Airplane(plane) => plane.fuel(),
        }
    }
}

fn main() {
    let car = Transport::Car(Car {
        speed: 120.0,
        fuel: 35.5,
    });
    let airplane = Transport::Airplane(Airplane {
        speed: 900.0,
        fuel: 2000.0,
    });

    println!(
        "Car speed: {} km/h, fuel: {} liters",
        car.speed(),
        car.fuel()
    );
    println!(
        "Airplane speed: {} km/h, fuel: {} liters",
        airplane.speed(),
        airplane.fuel()
    );
}
