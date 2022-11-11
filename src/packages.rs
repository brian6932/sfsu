use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

use crate::get_scoop_path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    /// The version of the package
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallManifest {
    /// The bucket the package was installed from
    pub bucket: String,
}

pub fn is_installed(manifest_name: impl AsRef<Path>, bucket: Option<impl AsRef<str>>) -> bool {
    let scoop_path = get_scoop_path();
    let installed_path = scoop_path
        .join("apps")
        .join(manifest_name)
        .join("current/install.json");

    if installed_path.exists() {
        if let Some(bucket) = bucket {
            let mut buf = String::new();

            File::open(installed_path)
                .unwrap()
                .read_to_string(&mut buf)
                .unwrap();

            let manifest: InstallManifest = serde_json::from_str(&buf).unwrap();

            manifest.bucket == bucket.as_ref()
        } else {
            true
        }
    } else {
        false
    }
}