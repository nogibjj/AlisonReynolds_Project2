use plotters::prelude::*;

// function to read in pressure.csv and return the temperature and pressure data
pub fn read_pressure_data() -> (Vec<f64>, Vec<f64>) {
    let mut rdr = csv::Reader::from_path("pressure.csv").unwrap();
    let mut temp = Vec::new();
    let mut pressure = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        temp.push(record[0].parse::<f64>().unwrap());
        pressure.push(record[1].parse::<f64>().unwrap());
    }
    (temp, pressure)
}

// function that takes vector as input and outputs the log transformed vector
pub fn log_transform(data: &Vec<f64>) -> Vec<f64> {
    let mut log_data = Vec::new();
    for i in 0..data.len() {
        log_data.push(data[i].ln());
    }
    log_data
}

// function that takes vector as input and outputs the square root of the vector
pub fn sqrt_transform(data: &Vec<f64>) -> Vec<f64> {
    let mut sqrt_data = Vec::new();
    for i in 0..data.len() {
        sqrt_data.push(data[i].sqrt());
    }
    sqrt_data
}

pub fn plot(x: Vec<f64>, y: Vec<f64>) {
    let data: Vec<(f64, f64)> = x.iter().cloned().zip(y.iter().cloned()).collect();
    let root_area = BitMapBackend::new("scatter.png", (700, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // calculate min and max values for x and y
    let mut min_x = x[0];
    let mut max_x = x[0];
    let mut min_y = y[0];
    let mut max_y = y[0];
    for i in 0..x.len() {
        if x[i] < min_x {
            min_x = x[i];
        }
        if x[i] > max_x {
            max_x = x[i];
        }
        if y[i] < min_y {
            min_y = y[i];
        }
        if y[i] > max_y {
            max_y = y[i];
        }
    }

    // find min of min_x and min_y
    let mut min = min_x;
    if min_y < min_x {
        min = min_y;
    }
    // find max of max_x and max_y
    let mut max = max_x;
    if max_y > max_x {
        max = max_y;
    }

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 55.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 55.0)
        .set_label_area_size(LabelAreaPosition::Right, 55.0)
        .set_label_area_size(LabelAreaPosition::Top, 55.0)
        .caption("Scatter Plot", ("sans-serif", 45.0))
        .build_cartesian_2d(min..max, min..max)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 4.0_f64, BLUE)))
        .unwrap();
    root_area.present().unwrap();
    println!("Plot finished");
}
