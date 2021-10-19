use serde::{Deserialize, Serialize};
use serde_json::Value;

use ureq::Agent;

#[derive(Debug)]
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: String,
    pub time: String,
    pub releaseTime: String,
}


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

pub fn get_version_manifest() -> Result<VersionManifest, std::io::Error> {
    let resp = Agent::default()
        .build()
        .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .set("Connection", "close")
        .call();

    if resp.status() == 200 {
        resp
            .into_json_deserialize::<VersionManifest>()
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("HTTP error {}", resp.status())))
    }
}



#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Arguments {
    game: Vec<Value>,
    jvm: Vec<Value>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: usize,
    totalSize: usize,
    url: String,
}


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: usize,
    url: String,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LibDownload {
    artifact: Artifact,
    classifiers: Option<Value>,
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Library {
    downloads: LibDownload,
    name: String,
    rules: Option<Vec<Value>>,
    natives: Option<Value>,
    extract: Option<Value>,
}



#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct VersionInfo {
    id: String,
    arguments: Arguments,
    assetIndex: AssetIndex,
    assets: String,
    complianceLevel: usize,
    // Downloads for client and server jars
    downloads: Value,
    logging: Value,
    libraries: Vec<Library>,
    mainClass: String,
    minimumLauncherVersion: usize,
    releaseTime: String,
    time: String,
    r#type: String,
}

pub fn download_artifact(a: &Artifact) {

}

pub fn get_version_info(url: &str) -> Result<VersionInfo, std::io::Error> {
    let resp = Agent::default()
        .build()
        .get(url)
        .set("Connection", "close")
        .call();

    if resp.status() == 200 {
        resp
            .into_json_deserialize::<VersionInfo>()
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("HTTP error {}", resp.status())))
    }
}

pub fn get_1_16_info() -> Result<VersionInfo, std::io::Error> {
    get_version_info("https://launchermeta.mojang.com/v1/packages/e54eda49b4a3b6dc407b952e494d0c32da422693/1.16.4.json")
}
