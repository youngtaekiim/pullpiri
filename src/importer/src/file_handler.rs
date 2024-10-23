/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
use ssh2::Session;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn get_absolute_file_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let file = Path::new(path);
    if !file.is_file() {
        return Err(format!("Not found or invalid path - {path}").into());
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        Ok(std::fs::canonicalize(file)?)
    } else {
        Err("Unsupported file extension".into())
    }
}

fn make_kube_file(dir: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let kube_file_path = format!("{}/{}.kube", dir, name);
    let yaml_file_path = format!("{}/{}.yaml", dir, name);
    let mut kube_file = File::create(kube_file_path)?;

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

fn make_yaml_file(dir: &str, name: &str, model: &str) -> Result<(), Box<dyn Error>> {
    let yaml_file_path = format!("{}/{}.yaml", dir, name);
    let mut yaml_file = File::create(yaml_file_path)?;

    yaml_file.write_all(model.as_bytes())?;

    Ok(())
}

pub fn perform(
    model_name: &str,
    parsed_model_str: &str,
    package_name: &str,
) -> Result<(), Box<dyn Error>> {
    let directory = format!(
        "{}/packages/{}/{}",
        common::get_conf("YAML_STORAGE"),
        package_name,
        model_name
    );
    std::fs::create_dir_all(&directory)?;

    make_kube_file(&directory, model_name)?;
    make_yaml_file(&directory, model_name, parsed_model_str)?;

    Ok(())
}

pub async fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let username = "admin";
    let password = Some("admin123".to_string());
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .basic_auth(username, password)
        .send()
        .await?;
    //let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let mut file = BufWriter::new(File::create(path)?);
        let content = response.bytes().await?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        println!("File downloaded to {}", path);
        Ok(())
    } else {
        Err(format!("Failed to download file: {}", response.status()).into())
    }
}

pub fn extract(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut archive: tar::Archive<File> = tar::Archive::new(file);

    let destination = Path::new(path).parent().unwrap();
    archive.unpack(destination)?;
    println!("TAR file extracted to {:?}", destination);
    Ok(())
}

pub fn copy_to_remote_node(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let guest_ssh_ip = common::get_conf("GUEST_SSH_IP");
    let tcp = std::net::TcpStream::connect(guest_ssh_ip)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    let (id, pw) = (
        common::get_conf("GUEST_NODE_ID"),
        common::get_conf("GUEST_NODE_PW"),
    );
    session.userauth_password(&id, &pw).unwrap();
    if !session.authenticated() {
        println!("auth failed to remote node");
        return Err("auth failed".into());
    }

    let local_folder = Path::new(path);
    let remote_folder = path;
    upload_to_remote_node(&session, local_folder, remote_folder)?;

    Ok(())
}

fn upload_to_remote_node(
    session: &Session,
    local_path: &Path,
    remote_path: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(local_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let remote_file_path = format!("{}/{}", remote_path, file_name.to_string_lossy());
        let rfp = Path::new(&remote_file_path);
        if path.is_dir() {
            session.sftp()?.mkdir(rfp, 0o755)?;
            upload_to_remote_node(session, &path, &remote_file_path)?;
        } else {
            let mut remote_file = session.sftp()?.create(rfp)?;
            let mut local_file = fs::File::open(&path)?;
            io::copy(&mut local_file, &mut remote_file)?;
        }
    }
    Ok(())
}
