fn main() {
    let temps = [
        -5.2,
        0.0,
        15.2,
        25.8,
        65.0,
        33.4,
        55.2,
        85.9,
        23.4,
        35.3,
        64.7
        ];

    for temperature in temps.iter() {
        let converted = convert_farenheit_to_celcius(temperature);
        println!("{0:.2} farenheit is {1:.2} celcius", temperature, converted);
    }
}

fn convert_farenheit_to_celcius(farenheit: &f64) ->f32 {
    let temp = *farenheit as f32;
    let multiplier: f32 = 5.0/9.0;
    let var: f32 = (temp - 32.0) * multiplier;
    var
}
