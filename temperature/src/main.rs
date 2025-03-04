fn main() {

    fn calculate_average(temps: &[f32]) -> Option<f32> {

        if temp.is_empty() {
            None
        } else {
            let mut sum: f32 = 0.0;

            for temperature in temps {
            sum += temperature;
        }
        let average = sum / temps.len() as f32;

        some(average)
        }
    }

    fn categorize_temp(temps: f32) -> &'static str {
        if temps < 10.0 {
            "cold"
        } else if temps <= 20.0 {
            "tempéré"
        } else {
            "hot"
        }
    }  

    struct AverageTemperature {
        average: f32,
        category: String,
    }


    let temperatures : [f32; 7] = [22.0, 19.5, 21.0, 23.5, 20.0, 18.0, 25.0];
    let average = calculate_average(&temperatures);
    let category = categorize_temp(average);
    println!("Moyenne température : {:.2}°C category: {}", average, category);

}
