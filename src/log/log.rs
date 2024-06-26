use crate::config::config::Server;
use crate::global::global::{OPEN_LOG_FILE_FAILED, WRITE_LOG_FILE_FAILED};
use crate::utils::{file, time};
use std::{
    fmt,
    fs::{File, OpenOptions},
    io::Write,
    path, sync,
};

const WRITE_MUTEX: sync::Mutex<()> = sync::Mutex::new(());

/**
 * 无输出写入
 */
pub fn write_no_print<T: fmt::Display + fmt::Debug>(server: &Server, log_msg: &T) {
    let now: String = time::format_now_time();
    let log_msg: &String = &format!("[{}]\n{:#?}", now, *log_msg);
    write(server, log_msg);
}

/**
 * 写入
 */
pub fn write(server: &Server, log_msg: &str) {
    let write_mutex: sync::Mutex<()> = WRITE_MUTEX;
    let lock: sync::MutexGuard<()> = match write_mutex.lock() {
        Ok(tem_lock) => tem_lock,
        _ => {
            return;
        }
    };
    let mut log_dir_path: String = server.log_dir_path.clone();
    if let Some(unix_path_str) = path::PathBuf::from(&log_dir_path).to_str() {
        log_dir_path = unix_path_str.replace("\\", "/");
    }
    if log_dir_path.ends_with('/') {
        log_dir_path.pop();
    }
    file::judge_creat_dir(&log_dir_path);
    let file_log_path: String = format!("{}/{}.log", log_dir_path, time::format_now_day());
    let mut file: File = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_log_path)
        .expect(OPEN_LOG_FILE_FAILED);
    writeln!(file, "{}\n", &log_msg).expect(WRITE_LOG_FILE_FAILED);
    drop(lock);
}
