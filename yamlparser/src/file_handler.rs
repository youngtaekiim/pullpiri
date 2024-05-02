use common::{Action, KubePod};
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn get_absolute_file_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file = Path::new(path);
    if !file.is_file() {
        return Err(format!("Not found or invalid path - {path}").into());
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        Ok(fs::canonicalize(file)?)
    } else {
        Err("Unsupported file extension".into())
    }
}

fn make_kube_file(dir: &str, name: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let kube_file_path = format!("{}/{}_{}.kube", dir, name, version);
    let yaml_file_path = format!("{}/{}_{}.yaml", dir, name, version);
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

fn make_yaml_file(
    dir: &str,
    name: &str,
    version: &str,
    action: &Action,
) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_file_path = format!("{}/{}_{}.yaml", dir, name, version);
    let mut yaml_file = fs::File::create(yaml_file_path)?;

    let kube_pod = KubePod::new(name, action.clone());

    let yaml_contents = serde_yaml::to_string(&kube_pod)?;
    yaml_file.write_all(yaml_contents.as_bytes())?;

    Ok(())
}

pub fn perform(name: &str, action: &Action) -> Result<(), Box<dyn std::error::Error>> {
    let directory = format!("{}{}", common::YAML_STORAGE, name);
    fs::create_dir_all(&directory)?;

    let image = action.get_image();

    let version = image
        .split(':')
        .collect::<Vec<&str>>()
        .last()
        .copied()
        .ok_or("cannot find version")?;

    make_kube_file(&directory, name, version)?;
    make_yaml_file(&directory, name, version, action)?;

    Ok(())
}
