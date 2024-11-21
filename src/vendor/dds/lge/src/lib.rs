// SPDX-License-Identifier: Apache-2.0
#![allow(non_snake_case)]

//pub mod batterycapacity;
//pub mod chargingstatus;
//pub mod daytime;
//pub mod gearstate;

pub mod vehicle_interface;

#[derive(Debug, Clone)]
pub struct DdsData {
    pub name: String,
    pub value: String,
}

use tokio::sync::mpsc::Sender;
pub async fn run(tx: Sender<DdsData>) {
    //tokio::spawn(batterycapacity::run(tx.clone()));
    //tokio::spawn(chargingstatus::run(tx.clone()));
    tokio::spawn(vehicle_interface::powertrain::PowertrainTransmission::run(tx.clone()));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::Path;

    #[test]
    pub fn idl2rs() {
        let src = Path::new("./src/idl");
        let dst = Path::new("./src");

        assert!(copy_dir(src, dst).is_ok());
    }

    pub fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let mut dest_path = dst.join(file_name);

            if entry_path.is_dir() {
                copy_dir(&entry_path, &dest_path)?;
            } else if entry_path.extension().unwrap() == "idl" {
                println!("{:?}  {:?}", entry_path, dest_path);
                let contents = get_contents_string(&entry_path);
                dest_path.set_extension("rs");
                fs::write(dest_path, contents)?;
            }
        }
        Ok(())
    }

    pub fn get_contents_string(src: &Path) -> String {
        let idl_src = fs::read_to_string(src).unwrap();
        let result = dust_dds_gen::compile_idl(&idl_src).unwrap();
        //println!("{}", result);
        result
    }
}
