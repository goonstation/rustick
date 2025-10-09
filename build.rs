use std::{
    fs::{File, read_to_string},
    io::Write,
};

macro_rules! feature_dm_file {
    ($name:expr) => {
        &"dmsrc/{}.dm".replace("{}", $name)
    };
}

fn main() {
    let mut f = File::create("target/rustick.dm").unwrap();

    writeln!(f, "{}", read_to_string(feature_dm_file!("main")).unwrap()).unwrap();
}
