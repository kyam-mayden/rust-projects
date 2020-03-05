fn main() {
    let farenheit_temps = [
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

    let celcius_temps: [f32; 4] = [1.4, -6.5, 55.1, 67.2];

    for temperature in farenheit_temps.iter() {
        let converted = convert_farenheit_to_celcius(&temperature);
        println!("{0:.2} farenheit is {1:.2} celcius", temperature, converted);
    }

    println!();

    for temperature in celcius_temps.iter() {
        let converted = convert_celcius_to_farenheit(&temperature);
        println!("{0:.2} celcius is {1:.2} farenheit", temperature, converted);
    }
}

fn convert_farenheit_to_celcius(farenheit: &f32)->f32 {
    let multiplier: f32 = 5.0/9.0;
    let new_temp: f32 = (farenheit - 32.0) * multiplier;
    new_temp
}

fn convert_celcius_to_farenheit(celcius: &f32)->f32 {
    let multiplier: f32 = 9.0/5.0;
    let var: f32 = (celcius * multiplier) + 32.0;
    var
}
