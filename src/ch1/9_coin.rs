fn main() {
    let original: f64 = 3950.0;
    let mut price: f64 = original;
    let max_500: f64 = 10.0;
    let max_100: f64 = 3.0;
    let max_50: f64 = 10.0;

    let num_500 = (price / 500.0).floor().min(max_500);

    price = price - num_500 * 500.0;

    let num_100 = (price / 100.0).floor().min(max_100);

    price = price - num_100 * 100.0;

    let num_50 = (price / 50.0).floor().min(max_50);

    let total = num_500 * 500.0 + num_100 * 100.0 + num_50 * 50.0;

    if original == total {
        println!("{}, {}, {}", num_500, num_100, num_50);
    } else {
        println!("{}", total - original);
    }
}
