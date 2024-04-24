use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};

macro_rules! Err {
    ($x:expr) => {
        Err(Error::new(ErrorKind::Other, $x))
    };
}

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

fn make_kube_file(directory: &str, name: &str, version: &str) -> Result<()> {
    let kube_file_path = format!("{}/{}_{}.kube", directory, name, version);
    let yaml_file_path = format!("{}/{}_{}.yaml", directory, name, version);
    let mut kube_file = fs::File::create(kube_file_path)?;

    let kube_contents = format!(
        r#"[Unit]
Description=A kubernetes yaml based {} service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml={}
"#,
        name, yaml_file_path
    );
    kube_file.write_all(kube_contents.as_bytes())?;

    Ok(())
}

fn make_yaml_file(directory: &str, name: &str, image: &str, version: &str) -> Result<()> {
    let yaml_file_path = format!("{}/{}_{}.yaml", directory, name, version);
    let mut yaml_file = fs::File::create(yaml_file_path)?;

    let yaml_contents = format!(
        r#"apiVersion: v1
kind: Pod
metadata:
  name: {0}
spec:
  containers:
  - name: {0}-container
    image: {1}
"#,
        name, image
    );
    yaml_file.write_all(yaml_contents.as_bytes())?;

    Ok(())
}

pub fn perform(name: &str, image: &str) -> Result<()> {
    let directory = format!("{}{}", common::YAML_STORAGE, name);
    fs::create_dir_all(&directory)?;

    let version = image
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or::<Error>(Error::new(ErrorKind::Other, "cannot find version"))?;

    make_kube_file(&directory, name, version)?;
    make_yaml_file(&directory, name, image, version)?;

    Ok(())
}
