use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;

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

fn check_src_file(name: &str) -> Result<String> {
    let file = Path::new(name);
    if !file.is_file() {
        return Err!(format!("Not found or invalid - {}", name));
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        let absolute_path = fs::canonicalize(file)?;
        Ok(absolute_path.into_os_string().into_string().unwrap())
    } else {
        Err!("Unsupported file extension")
    }
}

fn create_dst_file(name: &str) -> Result<()> {
    let kube_file_path = Path::new(name).with_extension("kube");
    let mut file = File::create(kube_file_path)?;

    file.write_all(format!("{}{}", CONTENTS_HEADER, name).as_bytes())?;
    Ok(())
}

fn delete_dst_files(name: &str) -> Result<()> {
    let yaml_file_path = Path::new(name);
    let kube_file_path = Path::new(name).with_extension("kube");

    fs::remove_file(yaml_file_path)?;
    fs::remove_file(kube_file_path)?;
    Ok(())
}

/// input example  : ./my_pod.yaml
/// output example : /etc/containers/systemd/my_pod.yaml
///                  /etc/containers/systemd/my_pod.kube
pub fn process(cmd: &str, file_path: &str) -> Result<()> {
    let src = check_src_file(file_path)?;
    let file_name = Path::new(&src).file_name().unwrap().to_str().unwrap();
    let dst = format!("{}{}", SYSTEMD_FILE_PATH, file_name);

    if cmd == "apply" {
        fs::copy(&src, &dst)?;
        create_dst_file(&dst)?;

        Ok(())
    } else if cmd == "delete" {
        delete_dst_files(&dst)?;

        Ok(())
    } else {
        Err!(format!("{} is not support", cmd))
    }
}
