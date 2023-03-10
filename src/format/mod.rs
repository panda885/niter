mod ext;

use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde::de::Unexpected;
use serde_json::Value;
use crate::project::{Project, Mod};
use crate::error::{Error, MapErrToNiterExt, Result};
use crate::format::ext::FromValueExt;

const SUPPORTED_FORMAT: &str = "0beta";

#[derive(Debug, Serialize, Deserialize)]
struct MainFile {
    format: String,
    name: String,
    version: String
}

#[derive(Debug)]
pub struct ProjectFormatter {
    path: PathBuf,
    main_file: MainFile
}


impl ProjectFormatter {

    pub fn format(path: PathBuf) -> Result<ProjectFormatter> {
        let main_file = format_main_file(path.join("niter.json"))?;
        Ok(ProjectFormatter {
            path,
            main_file
        })
    }

    pub fn create(path: PathBuf, project: &Project) -> Result<ProjectFormatter> {
        let main_file: MainFile = project.into();
        create_main_file(path.join("niter.json"), &main_file)?;

        Ok(ProjectFormatter {
            path,
            main_file
        })
    }

    pub fn mods_path(&self) -> PathBuf {
        self.path.join("mods")
    }

    pub fn mod_path(&self, name: &str) -> PathBuf {
        self.mods_path().join(name).with_extension("json")
    }

    pub fn mods(&self) -> Result<Vec<String>> {
        let mut mods: Vec<String> = Vec::new();
        let mods_path = self.mods_path();

        if mods_path.exists() {
            for entry in fs::read_dir(&mods_path).map_err_to_niter(&mods_path)? {
                let entry = entry.map_err_to_niter(&mods_path)?;
                let path = entry.path();

                if path.is_dir() || path.extension() != Some("json".as_ref()) {
                    continue;
                }

                mods.push(path.with_extension("").file_name().unwrap().to_os_string().into_string().unwrap());
            }
        }

        Ok(mods)
    }

    fn create_mods_dir(&self) -> Result<()> {
        let path = self.mods_path();
        if !path.exists() {
            fs::create_dir(self.mods_path()).map_err_to_niter(&path)?;
        }
        Ok(())
    }

    pub fn format_mod(&self, name: &str) -> Result<Mod> {
        let path = self.mod_path(name);

        let mut mod_data = serde_json::from_str::<Mod>(
            fs::read_to_string(&path)
                .map_err_to_niter(&path)?
                .as_str()
        ).map_err_to_niter(&path)?;

        mod_data.name = name.into();
        Ok(mod_data)
    }


    pub fn create_mod(&self, name: &str, mod_data: &Mod) -> Result<()> {
        self.create_mods_dir()?;

        let path = self.mod_path(name);
        serde_json::to_writer_pretty(
            fs::File::create(&path)
                .map_err_to_niter(&path)?,
            mod_data
        ).map_err_to_niter(&path)
    }

    pub fn remove_mod(&self, name: &str) -> Result<()> {
        let path = self.mod_path(name);
        fs::remove_file(&path).map_err_to_niter(&path)
    }
}


impl From<&Project> for MainFile {
    fn from(value: &Project) -> Self {
        MainFile {
            format: SUPPORTED_FORMAT.into(),
            name: value.name.clone(),
            version: value.version.clone()
        }
    }
}


fn format_main_file(path: PathBuf) -> Result<MainFile> {
    if !path.exists() {
        return Err(Error::MainFileNotFound);
    }

    let main_file: Value = serde_json::from_str(
        fs::read_to_string(&path)
            .map_err_to_niter(&path)?
            .as_str()
    ).map_err_to_niter(&path)?;

    let format = main_file
        .get("format")
        .ok_or_else(|| Error::Serde(serde::de::Error::missing_field("format")))?;
    let format = format
        .as_str()
        .ok_or_else(|| Error::Serde(serde::de::Error::invalid_type(Unexpected::from_value(format), &"a string")))?;

    if format != SUPPORTED_FORMAT {
        return Err(Error::UnsupportedFormat(format.into()));
    }

    Ok(serde_json::from_value(main_file).map_err_to_niter(&path)?)
}

fn create_main_file(path: PathBuf, main_file: &MainFile) -> Result<()> {
    if path.exists() {
        return Err(Error::AlreadyInitiated);
    }

    serde_json::to_writer_pretty(
        fs::File::create(&path).map_err_to_niter(&path)?,
        main_file
    ).map_err_to_niter(&path)
}

pub fn create_project(project: &Project, path: PathBuf) -> Result<()> {
    let formatter = ProjectFormatter::create(path, project)?;

    for mod_data in &project.mods {
        formatter.create_mod(mod_data.name.as_str(), mod_data)?;
    }

    Ok(())
}

pub fn format_project(path: PathBuf) -> Result<Project> {
    let formatter = ProjectFormatter::format(path)?;

    let mut mods: Vec<Mod> = Vec::new();
    for mod_name in formatter.mods()? {
        mods.push(formatter.format_mod(&mod_name)?)
    }

    Ok(Project {
        name: formatter.main_file.name,
        version: formatter.main_file.version,
        mods
    })
}
