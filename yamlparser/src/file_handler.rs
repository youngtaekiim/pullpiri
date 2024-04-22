use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

macro_rules! Err {
    ($x:expr) => {
        Err(Error::new(ErrorKind::Other, $x))
    };
}

const SYSTEMD_FILE_PATH: &str = r#"/etc/containers/systemd/"#;
const CONTENTS_HEADER: &str = r#"[Unit]
Description=A kubernetes yaml based pingpong service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml="#;

pub fn get_absolute_file_path(path: &str) -> Result<PathBuf> {
    let file = Path::new(path);
    if !file.is_file() {
        return Err!(format!("Not found or invalid path - {}", path));
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        Ok(fs::canonicalize(file)?)
    } else {
        Err!("Unsupported file extension")
    }
}

fn create_dst_file(file_name: &str) -> Result<()> {
    //create kube file to dst
    let mut dst_path = PathBuf::from(SYSTEMD_FILE_PATH);
    dst_path.push(format!("{}", file_name));
    let mut file = fs::File::create(Path::new(&dst_path).with_extension("kube"))?;
    let contents = format!("{}{}", CONTENTS_HEADER, dst_path.display());
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn update_yaml_file(new_image: &str, file_name: &str, version: &str) -> Result<()> {
    let mut src_path = PathBuf::from(SYSTEMD_FILE_PATH);
    src_path.push(format!("{}.yaml", file_name));
    println!("{}", src_path.display());

    let mut new_path = PathBuf::from(SYSTEMD_FILE_PATH);
    new_path.push(format!("{}_{}.yaml", file_name, version));
    println!("{}", new_path.display());

    let src_file = fs::File::open(src_path)?;
    let dst_file = fs::File::create(&new_path).expect("Failed to open file in write mode");
    let reader = BufReader::new(src_file);
    let mut writer = BufWriter::new(dst_file);
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

#[allow(dead_code)]
fn delete_dst_files(dst_yaml_path: &str) -> Result<()> {
    fs::remove_file(Path::new(dst_yaml_path))?;
    fs::remove_file(Path::new(dst_yaml_path).with_extension("kube"))?;
    Ok(())
}

pub fn perform(name: &str, operation: &str, image: &str, version: &str) -> Result<()> {
    let directory = Path::new(SYSTEMD_FILE_PATH).join(name);
    fs::create_dir_all(directory)?;

    match operation {
        "deploy" => Err!("deploy is not yet implemented"),
        "update" => update_yaml_file(image, name, version),
        "rollback" => Err!("rollback is not yet implemented"),
        _ => Err!("unsupported operation"),
    }
}
