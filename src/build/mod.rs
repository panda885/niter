use std::fs;
use std::path::PathBuf;
use crate::log;
use crate::error::{MapErrToNiterExt, Result};
use crate::project::Project;

pub fn build(project: &Project, path: PathBuf) -> Result<()> {
    build_installation(project, path.join("installation"))
}

pub fn build_installation(project: &Project, path: PathBuf) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            fs::remove_file(&path).map_err_to_niter(&path)?
        } else {
            fs::remove_dir_all(&path).map_err_to_niter(&path)?
        }
    }

    fs::create_dir_all(&path).map_err_to_niter(&path)?;

    let mods_dir = path.join("mods");
    fs::create_dir(&mods_dir).map_err_to_niter(&mods_dir)?;

    let client = reqwest::blocking::Client::builder()
        .build()?;

    for mod_data in &project.mods {
        let file_name = mod_data.file_or_source()?;
        let url = mod_data.source.url()?;

        log!("Downloading {}", file_name);

        let mod_path = mods_dir.join(&file_name);

        let response = client.get(url).send()?;
        let body = response.text()?;

        let mut file = fs::File::create(&mod_path).map_err_to_niter(&mod_path)?;

        std::io::copy(&mut body.as_bytes(), &mut file).map_err_to_niter(&mod_path)?;
    }

    Ok(())
}
