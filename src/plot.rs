use plotters::prelude::*;

pub fn draw_fitness(fitness_curve:Vec<f64>) {

    // Create a drawing area
    let root_area = BitMapBackend::new("fitness_curve.png", (800, 600))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // Define the chart
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Fitness Curve", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..fitness_curve.len(), 0.0..200.0)
        .unwrap();

    // Configure the chart
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Generation")
        .y_desc("Fitness")
        .draw()
        .unwrap();

    // Plot the fitness curve
    chart
        .draw_series(LineSeries::new(
            fitness_curve.iter().enumerate().map(|(i, &f)| (i, f)),
            &RED,
        ))
        .unwrap()
        .label("Fitness")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Configure the legend
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    // Save the plot as an image
    root_area.present().unwrap();
}
