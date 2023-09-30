enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T: Clone> Maybe<T> {
    // `is_just` メソッドを実装してください。
    // このメソッドは `Maybe::Just(_)` であれば true を、
    // `Maybe::Nothing` であれば false を返します。
    fn is_just(&self) -> bool {
        match self {
            Maybe::Just(_) => true,
            Maybe::Nothing => false,
        }
    }

    // `is_nothing` メソッドを実装してください。
    // このメソッドは `Maybe::Nothing` であれば true を、
    // `Maybe::Just(_)` であれば false を返します。
    fn is_nothing(&self) -> bool {
        match self {
            Maybe::Just(_) => false,
            Maybe::Nothing => true,
        }
    }

    // `unwrap_or` メソッドを実装してください。
    // このメソッドは `Maybe::Just(_)` であればその値を、
    // `Maybe::Nothing` であれば引数として渡されたデフォルト値を返します。
    fn unwrap_or(&self, value: T) -> T {
        match self {
            Maybe::Just(v) => v.clone(),
            Maybe::Nothing => value,
        }
    }
}

fn main() {
    let just_value: Maybe<i32> = Maybe::Just(42);
    let nothing_value: Maybe<i32> = Maybe::Nothing;

    // 上で定義したメソッドを使って、各ケースで適切な出力を行ってください。
    println!("{}", just_value.is_just());
    println!("{}", nothing_value.is_nothing());
    println!("{}", just_value.unwrap_or(42));
    println!("{}", nothing_value.unwrap_or(50));

    let just_value: Maybe<&str> = Maybe::Just("abc");
    let nothing_value: Maybe<&str> = Maybe::Nothing;
    println!("{}", just_value.is_just());
    println!("{}", nothing_value.is_nothing());
    println!("{}", just_value.unwrap_or("abc"));
    println!("{}", nothing_value.unwrap_or("def"));

    let just_value: Maybe<Vec<i32>> = Maybe::Just(vec![10, 20]);
    let nothing_value: Maybe<Vec<i32>> = Maybe::Nothing;
    println!("{}", just_value.is_just());
    println!("{}", nothing_value.is_nothing());
    println!("{:?}", just_value.unwrap_or(vec![10, 20]));
    println!("{:?}", nothing_value.unwrap_or(vec![0, 0]));
}
