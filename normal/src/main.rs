#![windows_subsystem = "windows"]
use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn main() {
    let dir_name = env::current_dir().unwrap().display().to_string();
    let path_arr = dir_name.split("\\").collect::<Vec<&str>>();
    const EXE_NAME: &str = "league-stop.exe";
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
            fs::copy(
                format!("{}/{}", dir_name, EXE_NAME),
                format!("{}/{}", path, EXE_NAME),
            )
            .unwrap();
        }
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let command = Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("LeagueClient.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
        println!("{:?}", command);
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
