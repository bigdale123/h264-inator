use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;

fn check_for_h264(video: &Path) -> bool {
    let ffprobe_command = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=codec_name")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(video)
        .output();

    // println!("{}", ffprobe_command.as_ref().expect("No Output from Command.").stdout.len());
    if ffprobe_command.as_ref().expect("No Output from Command.").stdout.len() > 0 {
        let output = ffprobe_command.expect("No Output from Command.").stdout.clone();
        let codec_name = String::from_utf8_lossy(&output);
        return codec_name.trim() == "h264"
    }
    else {
        return false;
    }
    
}

fn get_videos(directory: &Path) -> Vec<PathBuf> {
    let mut videos: Vec<PathBuf> = Vec::new();

    let valid_extension = [
        String::from("mp4"),
        String::from("mkv"),
        String::from("avi"),
        String::from("mov"),
        String::from("wmv"),
        String::from("flv"),
        String::from("webm"),
    ];

    if let Ok(files) = fs::read_dir(directory) {
        for file in files {
            if let Ok(file) = file {
                let path = file.path();
                if path.is_dir() {
                    videos.extend(get_videos(&path));
                }
                else if valid_extension.contains(&path.extension().expect("No Extension Found").to_string_lossy().to_lowercase()) && !check_for_h264(&path) {
                    videos.push(path.clone());
                }
            }
        }
    }

    return videos;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Too few arguments provided!");
        println!("Usage: ./h264-inator <folder_paths> ...");
        return;
    }
    for i in 1..args.len() {
        if args[i].len() > 0 {
            let directory = Path::new(&args[i]);
            let videos = get_videos(directory);
        
            println!("The Following files are NOT in h264.");
            for video in &videos {
                println!("  {}", video.display());
            }
            println!("Total Files: {}", videos.len());
        }        
    }
}