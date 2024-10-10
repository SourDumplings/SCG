use log::{debug, error, info, trace, warn};
use log4rs::{
    append::console::ConsoleAppender,
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::env;

pub fn init_logger()
{
    // 获取可执行文件的路径
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();

    // 创建 log 目录路径
    let log_dir = exe_dir.join("log");
    std::fs::create_dir_all(&log_dir).unwrap();

    // 创建控制台输出
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {l} {T}:{I} {M} {f}:{L} - {m}{n}",
        )))
        .build();

    // 创建第一个文件输出，按天和大小滚动
    let app_log_path = log_dir.join("scg_app.log");
    let window_roller_app = FixedWindowRoller::builder()
        .build("log/scg_app.{}.log", 7)
        .unwrap();
    let size_trigger_app = SizeTrigger::new(10 * 1024 * 1024); // 10 MB
    let compound_policy_app =
        CompoundPolicy::new(Box::new(size_trigger_app), Box::new(window_roller_app));
    let scg_app_log_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {l} {T}:{I} {M} {f}:{L} - {m}{n}",
        )))
        .build(
            app_log_path.to_str().unwrap(),
            Box::new(compound_policy_app),
        )
        .unwrap();

    // 创建第二个文件输出，按天和大小滚动
    let stat_log_path = log_dir.join("scg_stat.log");
    let window_roller_stat = FixedWindowRoller::builder()
        .build("log/scg_stat.{}.log", 7)
        .unwrap();
    let size_trigger_stat = SizeTrigger::new(10 * 1024 * 1024); // 10 MB
    let compound_policy_stat =
        CompoundPolicy::new(Box::new(size_trigger_stat), Box::new(window_roller_stat));
    let scg_stat_log_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
        .build(
            stat_log_path.to_str().unwrap(),
            Box::new(compound_policy_stat),
        )
        .unwrap();

    // 设置根日志级别
    let root_level = if cfg!(debug_assertions)
    {
        log::LevelFilter::Debug
    }
    else
    {
        log::LevelFilter::Info
    };

    // 配置日志
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("scg_app", Box::new(scg_app_log_file)))
        .appender(Appender::builder().build("scg_stat", Box::new(scg_stat_log_file)))
        .logger(
            Logger::builder()
                .appender("scg_stat")
                .additive(false)
                .build("stat", log::LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("scg_app")
                .build(root_level),
        )
        .unwrap();

    // 初始化日志配置
    log4rs::init_config(config).unwrap();
}

pub fn log_stat(msg: &str)
{
    log::logger().log(
        &log::Record::builder()
            .args(format_args!("{}", msg))
            .level(log::Level::Info)
            .target("stat")
            .build(),
    );
}

pub fn test_log()
{
    trace!("This is a trace message");
    debug!("This is a debug message");
    info!("This is an info message");
    warn!("This is an warn message");
    error!("This is an error message");

    log_stat("This is a stat message");
}
