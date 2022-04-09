#![windows_subsystem = "windows"]
use std::env;
use std::fs;
use std::process::Command;

#[tokio::main]
async fn main() {
    let dir_name = env::current_dir().unwrap().display().to_string();
    let path_arr = dir_name.split("\\").collect::<Vec<&str>>();
    const EXE_NAME: &str = "league-stop-weak.exe";
    let path = format!(
        "{}/{}/{}/{}",
        path_arr[0],
        path_arr[1],
        path_arr[2],
        "AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup"
    );
    loop {
        let dirs = fs::read_dir(&path).unwrap();
        let mut is_in_start = false;
        for item in dirs {
            let file = item.unwrap().path().display().to_string();
            if file.contains(EXE_NAME) {
                is_in_start = true;
            }
        }
        if is_in_start == false {
            fs::copy(EXE_NAME, format!("{}/{}", path, EXE_NAME)).unwrap();
        }
        Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("League of Legends.exe")
            .output()
            .expect("Failed to execute command");
        // let output = reqwest::get("https://127.0.0.1:2999/liveclientdata/allgamedata").await;
        // if output.unwrap_err().to_string().contains("-2146762487") {
        //     Command::new("taskkill")
        //         .arg("/F")
        //         .arg("/IM")
        //         .arg("League of Legends.exe")
        //         .output()
        //         .expect("Failed to execute command");
        // }
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
