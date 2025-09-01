use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

/*
[dependencies]
clap = { version = "3.2", features = ["derive"] }
*/

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        eprintln!("请提供至少一个文件或文件夹路径作为参数\n\n
        本软件用于给视频批量生成预览图，请把视频文件或文件夹拖到本软件图标上即可，支持拖多个过来\n
        暂只支持MP4和MKV格式的视频文件\n
        本窗口15秒后自动退出");
        sleep(Duration::from_secs(15));
        std::process::exit(1);
    }

    let mpc_path = r"C:\Program Files\MPC-HC\mpc-hc64.exe";
    if !Path::new(mpc_path).exists() {
        eprintln!("找不到 MPC-HC 执行文件: {}", mpc_path);
        std::process::exit(1);
    }

    let video_exts = ["mp4", "mkv"];
    let mut video_files = Vec::new();

    for arg in args {
        let path = Path::new(&arg);
        
        if !path.exists() {
            eprintln!("路径不存在: {}", arg);
            continue;
        }

        if path.is_file() {
            if is_video_file(path, &video_exts) {
                if let Ok(absolute_path) = path.canonicalize() {
                    video_files.push(absolute_path);
                }
            } else {
                eprintln!("跳过非视频文件: {}", arg);
            }
        } else if path.is_dir() {
            find_video_files(path, &video_exts, &mut video_files);
        }
    }

    let total_files = video_files.len();
    let mut file_count = 1;

    for video_path in video_files {
        let path_str = video_path.to_string_lossy();
        let path_str = &path_str[4..];
        println!("[{}/{}] 处理视频: {}", file_count, total_files, path_str);
        file_count += 1;

        let status = Command::new(mpc_path)
            .arg(path_str.to_string())
            .arg("/thumbnails")
            .arg("/minimized")
            .status()
            .expect("执行命令失败");
        
        if !status.success() {
            eprintln!("处理失败: {}", path_str);
        }
    }
}

fn is_video_file(path: &Path, exts: &[&str]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| exts.iter().any(|&e| ext.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

fn find_video_files(dir: &Path, exts: &[&str], results: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_video_files(&path, exts, results);
            } else if is_video_file(&path, exts) {
                if let Ok(absolute_path) = path.canonicalize() {
                    results.push(absolute_path);
                }
            }
        }
    }
}