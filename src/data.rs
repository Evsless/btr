use std::fs::{self, File};
use std::io;
use home;

pub struct DataHandler {
    root: Option<File>
}

impl DataHandler {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }


    pub fn data_init(&mut self) -> Result<(), io::Error> {
        let home_dir = match home::home_dir() {
            Some(home_path) => home_path.into_os_string().into_string().unwrap(),
            _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Home directory wasn't found"))
        };

        match fs::exists(format!("{}/.btr", home_dir)) {
            Ok(exists) => {
                if false == exists {
                    match fs::create_dir(format!("{}/.btr", home_dir)) {
                        Err(e) => return Err(e),
                        _ => {}
                    }
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        self.root = Some(File::create(format!("{}/.btr/log.json", home_dir))?);

        Ok(())
    }
}