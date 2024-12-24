use csv::Writer;


// 乱数を生成してCSVファイルに保存する
fn main() -> Result<(), Box<dyn std::error::Error>> {
        // ---- 初期化する ----
    // 乱数生成器
    let mut generator = rand_simple::Cauchy::new(1192_u32);
    // CSVファイルのパス
    let file_path = "C:/MyPrograms/demo_rand_simple/data/Cauchy.csv";


    // ---- 乱数を生成する ----
    // 標準分布
    let mut standard_samples = Vec::<f64>::new();
    for _ in 0..1000 {
        standard_samples.push(generator.sample() as f64);
    }

    // パラメータを変更した分布
    let location: f64 = -2_f64;
    let scale: f64 = 1.5_f64;
    let _: Result<(f64, f64), &str> = generator.try_set_params(location, scale);
    let mut custom_samples = Vec::<f64>::new();
    for _ in 0..1000 {
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
