use log::info;

pub async fn logger_init(logger_path: String ){

    let date = chrono::Local::now().format("%Y-%m-%d");
    let logfile_path = format!("{}/logs/{}.log", logger_path, date);

    if !tokio::fs::metadata(&logger_path).await.is_ok() {
        match tokio::fs::create_dir_all(&logger_path).await {
            Ok(()) => info!("Directory created successfully {}", logfile_path),
            Err(e) => info!("Failed to create directory: {}", e),
        }
    }

    // println!("{}", logfile_path);
    let mut dispatch = fern::Dispatch::new();
    dispatch = dispatch
        .format(|out, message, record| {
            let file = record.file().unwrap_or("<unknown>");
            let line = record.line().unwrap_or(0);
            out.finish(format_args!(
                "[{}] [{}] [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                file,
                line,
                message
            ))
        })
        .level(log::LevelFilter::Info)
        // .level(log::LevelFilter::Error)
        // .level(log::LevelFilter::Trace)
        // .level(log::LevelFilter::Warn)
        // .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(logfile_path).expect("Failed to create log file"));
    dispatch.apply().unwrap();

    info!("init logger config success")
}