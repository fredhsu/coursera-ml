use plotters::prelude::*;

fn c1_w1_lab02() {
    let x_train: Vec<f32> = vec![1.0, 2.0];
    let y_train = vec![300.0, 500.0];
    println!("x_train = {:#?}", x_train);
    println!("y_train = {:#?}", y_train);
    let m = x_train.len();
    println!("The number of training examples is: {m}");
    let i = 0;
    let x_i = x_train[i];
    let y_i = y_train[i];
    println!("(x^(0), y^(0)) = ({x_i}, {y_i})");
    // plot the data points
    let root = BitMapBackend::new("images/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0f32..2.0f32, 200.0f32..600.0f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(LineSeries::new(
            (x_train)
                .clone()
                .into_iter()
                .map(|x| (x, 100.0 * x + 100.0)),
            &GREEN,
        ))
        .unwrap();
    //root.present().unwrap();
    /*
         * # Plot the data points
    plt.scatter(x_train, y_train, marker='x', c='r')
    # Set the title
    plt.title("Housing Prices")
    # Set the y-axis label
    plt.ylabel('Price (in 1000s of dollars)')
    # Set the x-axis label
    plt.xlabel('Size (1000 sqft)')
    plt.show()
         */
    let w = 100.0;
    let b = 100.0;
    println!("w: {w}, b: {b}");
    let f_wb = compute_model_output(x_train, w, b);
    dbg!(f_wb);
    let w = 200.0;
    let b = 100.0;
    println!("w: {w}, b: {b}");
    let x_i = 1.2;
    let cost_1200sqft = w * x_i + b;
    println!("cost_1200sqft: {cost_1200sqft} thousand dollars");
}

// Computes the predition of a linear model
fn compute_model_output(x: Vec<f32>, w: f32, b: f32) -> Vec<f32> {
    let m = x.len();
    let mut f_wb: Vec<f32> = vec![0.0; m];
    for (i, x_i) in x.iter().enumerate() {
        f_wb[i] = w * x_i + b;
    }
    f_wb
}

fn compute_cost(x: Vec<f32>, y: Vec<f32>, w: f32, b: f32) -> f32 {
    let m = x.len() as f32;
    let mut cost_sum = 0.0;
    for (i, x_i) in x.iter().enumerate() {
        let f_wb = w * x_i + b;
        let cost = (f_wb - y[i]).powi(2);
        cost_sum += cost;
    }
    (1.0 / (2.0 * m)) * cost_sum
}
fn c1_w1_lab03() {
    let x_train = vec![1.0, 1.7, 2.0, 2.5, 3.0, 3.2];
    let y_train = vec![250.0, 300.0, 480.0, 430.0, 630.0, 730.0];
    let total_cost = compute_cost(x_train, y_train, 209.0, 2.4);
    dbg!(total_cost);
}

fn main() {
    c1_w1_lab02();
    c1_w1_lab03();
}
