mod utils {
    use std::{
        env::consts::OS,
        fs::{File, OpenOptions},
        io::Write,
        path::PathBuf,
    };

    /// Returns the path to the user's AppData directory on Windows
    /// # Returns
    /// Option<PathBuf>
    #[allow(dead_code)]
    fn get_app_data_dir() -> Option<PathBuf> {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    }

    /// Returns the path to the user's .local/share directory on Linux
    /// # Returns
    /// Option<PathBuf>
    #[allow(dead_code)]
    fn get_local_share_dir() -> Option<PathBuf> {
        std::env::var("XDG_DATA_HOME").ok().map(PathBuf::from)
    }

    /// Returns the path to the user's Library directory on macOS
    /// # Returns
    /// Option<PathBuf>
    #[allow(dead_code)]
    fn get_library_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }

    #[allow(dead_code)]
    pub fn get_log_dir() -> PathBuf {
        let oparating_sys = OS;
        let current_file_name: String = std::env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(" ", "_");

        let dir_path = match oparating_sys {
            "windows" => get_app_data_dir()
                .unwrap()
                .join(current_file_name)
                .join("logs"),
            "linux" => get_local_share_dir()
                .unwrap()
                .join(current_file_name)
                .join("logs"),
            // TODO: dont have a map to test with
            // "macos" => {
            //     get_library_dir().unwrap().join("logs").join(current_file_name)
            // },
            _ => panic!("Unknown oparating system: not supported :("),
        };

        let exists = std::fs::exists(&dir_path).unwrap();

        if !exists {
            std::fs::create_dir(&dir_path).unwrap();
        }

        dir_path
    }

    #[allow(dead_code)]
    pub fn log_to_file(string: String) {
        let path = get_log_dir().join("logs.txt");

        let metadata = std::fs::metadata(&path).unwrap();
        if metadata.len() == 10000 {
            // Open file in write mode and truncate (empty) it
            File::create(&path).unwrap(); // create truncates file
        } else {
            // Open file in append mode and write the string with newline
            let mut file = OpenOptions::new().append(true).open(&path).unwrap();
            writeln!(file, "{}", string).unwrap();
        }
    }
}
#[macro_export]
macro_rules! debug_log {
    ($flag:expr, $msg:expr) => {{
        let args: Vec<String> = ::std::env::args().collect();
        if args
            .iter()
            .any(|arg| arg == "--all" || arg == concat!("--", $flag))
        {
            let string = format!("{:?}:{:?}", $flag, $msg);

            #[cfg(debug_assertions)]
            {
                println!("{}", string);
            }
            #[cfg(not(debug_assertions))]
            {
                utils::log_to_file(string);
            }
        }
    }};
}

