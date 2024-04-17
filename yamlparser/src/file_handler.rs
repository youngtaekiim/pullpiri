use std::ffi::OsStr;
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

macro_rules! Err {
    ($x:expr) => {
        Err(Error::new(ErrorKind::Other, $x))
    };
}

const SYSTEMD_FILE_PATH: &str = r#"/etc/containers/systemd/test/"#;
const CONTENTS_HEADER: &str = r#"[Unit]
Description=A kubernetes yaml based pingpong service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml="#;

pub fn get_absolute_file_path(name: &str) -> Result<String> {
    let file = Path::new(name);
    if !file.is_file() {
        return Err!(format!("Not found or invalid path - {}", name));
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        let absolute_path = fs::canonicalize(file)?;
        Ok(absolute_path.into_os_string().into_string().unwrap())
    } else {
        Err!("Unsupported file extension")
    }
}

pub fn create_dst_file(file_name: &str) -> Result<()> {
    //create kube file to dst
    let mut dst_path = PathBuf::from(SYSTEMD_FILE_PATH);
    dst_path.push(format!("{}", file_name));
    let mut file = fs::File::create(Path::new(&dst_path).with_extension("kube"))?;
    let contents = format!("{}{}", CONTENTS_HEADER, dst_path.display());
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn update_yaml_file(new_image: &str, file_name: &str, version: &str) -> Result<()> {
    let mut src_path = PathBuf::from(SYSTEMD_FILE_PATH);
    src_path.push(format!("{}.yaml", file_name));
    println!("{}", src_path.display());

    let mut new_path = PathBuf::from(SYSTEMD_FILE_PATH);
    new_path.push(format!("{}_{}.yaml", file_name, version));
    println!("{}", new_path.display());

    let src_file = fs::File::open(src_path).unwrap();
    let dst_file = fs::File::create(&new_path).expect("Failed to open file in write mode");
    let reader = io::BufReader::new(src_file);
    let mut writer = io::BufWriter::new(dst_file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("image:") {
            lines.push(format!("image: {}", new_image));
        } else {
            lines.push(line);
        }
    }
    
    for line in &lines {
        writeln!(writer, "{}", line).expect("Failed to write line");
    }
    let _ = create_dst_file(&new_path.display().to_string())?;
    Ok(())
}

fn delete_dst_files(dst_yaml_path: &str) -> Result<()> {
    fs::remove_file(Path::new(dst_yaml_path))?;
    fs::remove_file(Path::new(dst_yaml_path).with_extension("kube"))?;
    Ok(())
}