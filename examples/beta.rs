use csv::Writer;

// 乱数を生成してCSVファイルに保存する
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- 初期化する ----
    // 乱数生成器
    let mut generator =
        rand_simple::Beta::new([1192u32, 765u32, 1543u32, 2003u32, 1867u32, 1688u32]);
    // CSVファイルのパス
    let file_path = "C:/MyPrograms/demo_rand_simple/data/Beta.csv";

    // ---- 乱数を生成する ----
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

    // ---- 生成した乱数をCSVファイルに保存する ----
    // CSVファイルWriterの初期化
    let mut wtr = Writer::from_path(file_path)?;
    // 列名の保存
    wtr.write_record(["Standard distribution", "Custom parameter distribution"])?;
    // 生成した乱数を保存する
    for (sd, cpd) in standard_samples.iter().zip(custom_samples.iter()) {
        wtr.write_record([sd, cpd].into_iter().map(|e| e.to_string()).clone())?;
    }
    // 書き込み完了時に行う
    wtr.flush()?;

    Ok(())
}
