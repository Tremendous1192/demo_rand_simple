use plotters::prelude::*;

/// Common functions to draw random numbers
/// * file_name_with_extension: e.g. "examples/bernoulli.png"
/// * caption: e.g. "Bernoulli distribution"
/// * standard_samples: Random number sequence generated with standard distribution
/// * custom_samples: Random number sequence
/// * x_range: e.g. -1_f64..3_f64
/// * x_step: e.g 1f64
/// * y_range: e.g 0..10_000
pub fn draw_histgram(
    file_name_with_extension: &str,
    caption: &str,
    standard_samples: &Vec<f64>,
    custom_samples: &Vec<f64>,
    x_range: std::ops::Range<f64>,
    x_step: f64,
    y_range: std::ops::Range<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    // キャンバスの生成
    let root = BitMapBackend::new(file_name_with_extension, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // キャンバスの設定
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .caption(caption, ("sans-serif", 50.0))
        .build_cartesian_2d(x_range.step(x_step).use_round().into_segmented(), y_range)?;

    // 軸の設定
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Random variable x")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    // 標準分布の描画
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.3).filled())
                .margin(1)
                .data(standard_samples.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap()
        .label("Standard distribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.3)));

    // パラメータを変更した分布の描画
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.3).filled())
                .margin(1)
                .data(custom_samples.iter().map(|x: &f64| (*x, 1))),
        )
        .unwrap()
        .label("Parameter change")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE.mix(0.3)));

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 乱数生成器
    let mut generator = rand_simple::Bernoulli::new(1192_u32);

    // 標準分布
    let mut standard_samples = Vec::<f64>::new();
    for _ in 0..1000 {
        standard_samples.push(generator.sample() as f64);
    }

    // パラメータを変更した分布
    let probability: f64 = 0.8f64;
    let _: Result<f64, &str> = generator.try_set_params(probability);
    let mut custom_samples = Vec::<f64>::new();
    for _ in 0..1000 {
        custom_samples.push(generator.sample() as f64);
    }

    // 乱数列ヒストグラムの描画
    let _ = draw_histgram(
        "examples/common_functions.png",
        "Bernoulli distribution",
        &standard_samples,
        &custom_samples,
        -1_f64..3_f64,
        1f64,
        0..1_000,
    );

    Ok(())
}
