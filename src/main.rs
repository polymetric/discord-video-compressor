use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

fn main() {
    let inst_id = Uuid::new_v4();
    let mut work_dir = "/tmp/discord_video_compressor/".to_string();
    work_dir.push_str(&inst_id.to_string());
    fs::create_dir_all(&work_dir).ok();

    Command::new("youtube-dl")
        .current_dir(&work_dir)
        .args(&["https://www.youtube.com/watch?v=mPbZIJst5eU"])
        .status()
        .expect("youtube-dl failed");

    let filepath = get_first_file(&work_dir).unwrap().to_str().unwrap().to_string();

    Command::new("ffmpeg")
        .current_dir(&work_dir)
		.args(&["-i", &filepath, "-c", "copy", "test.mp4"])
        .status()
        .expect("ffmpeg failed");
}

fn get_first_file<P: AsRef<Path>>(path: P) -> Result<PathBuf, io::Error> {
    Result::Ok(
        fs::read_dir(path)?
            .map(|result| result.map(|entry| entry.path()))
            .next()
            .unwrap()
            .unwrap(),
    )
}

