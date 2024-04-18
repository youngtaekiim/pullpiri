use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind, Result, Write};
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

fn get_absolute_file_path(name: &str) -> Result<PathBuf> {
    let file = Path::new(name);
    if !file.is_file() {
        return Err!(format!("Not found or invalid path - {}", name));
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        Ok(fs::canonicalize(file)?)
    } else {
        Err!("Unsupported file extension")
    }
}

fn create_dst_file(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    let mut file = fs::File::create(dst.with_extension("kube"))?;
    let dst_str = dst.clone().into_os_string().into_string();
    let dst_str = match dst_str {
        Ok(str) => str,
        Err(_) => return Err!("cannot determine yaml path string"),
    };

    file.write_all(format!("{}{}", CONTENTS_HEADER, dst_str).as_bytes())?;
    fs::copy(src, dst)?;
    Ok(())
}

fn delete_dst_files(dst: &PathBuf) -> Result<()> {
    fs::remove_file(dst)?;
    fs::remove_file(dst.with_extension("kube"))?;
    Ok(())
}

/// input example  : ./my_pod.yaml
/// output example : /etc/containers/systemd/my_pod.yaml
///                  /etc/containers/systemd/my_pod.kube
pub fn handle(cmd: &str, src_file_path: &str) -> Result<()> {
    let src = get_absolute_file_path(src_file_path)?;
    let file_name = Path::new(&src)
        .file_name()
        .ok_or::<Error>(Error::new(ErrorKind::Other, "cannot determine file name"))?;
    let dst = Path::new(SYSTEMD_FILE_PATH).join(file_name);

    if cmd == "apply" {
        create_dst_file(&src, &dst)?;
    } else if cmd == "delete" {
        delete_dst_files(&dst)?;
    } else {
        return Err!(format!("{} is not support", cmd));
    }
    Ok(())
}
