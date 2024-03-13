mod common_functions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 乱数生成器
    let mut generator =
        rand_simple::Beta::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32, 1688u32]);

    // 標準分布
    let mut standard_samples = Vec::<f64>::new();
    for _ in 0..10000 {
        standard_samples.push(generator.sample() as f64);
    }

    // パラメータを変更した分布
    let shape_alpha: f64 = 2f64;
    let shape_beta: f64 = 1.5f64;
    let _: Result<(f64, f64), &str> = generator.try_set_params(shape_alpha, shape_beta);
    let mut custom_samples = Vec::<f64>::new();
    for _ in 0..10000 {
        custom_samples.push(generator.sample() as f64);
    }

    // 乱数列ヒストグラムの描画
    let _ = common_functions::draw_histgram(
        "examples/beta.png",
        "Beta distribution",
        &standard_samples,
        &custom_samples,
        0f64..2f64,
        0.1f64,
        0..2_000,
    );

    Ok(())
}
