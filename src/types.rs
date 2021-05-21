//! This module contains all datatypes used by the cfwidget API

use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

/// Counts for project downloads
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct DownloadsMetric {
    /// Total downloads over the past month
    pub monthly: u64,

    /// Total downloads all time
    pub total: u64,
}

/// Info about a member of a project
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct ProjectMember {
    /// Project title for the user (ex. Owner)
    pub title: String,

    /// Username on CurseForge
    pub username: String,
}

/// Which stream a file is from
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReleaseStream {
    /// Full release
    #[serde(rename = "release")]
    Release,

    /// Beta release
    #[serde(rename = "beta")]
    Beta,
}

/// A file attached to a project
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectFile {
    /// File ID
    pub id: u32,

    /// File download URL
    pub url: Url,

    /// The "friendly name" of the file
    #[serde(rename = "display")]
    pub friendly_name: String,

    /// The real file name
    #[serde(rename = "name")]
    pub file_name: String,

    /// The file's release stream
    #[serde(rename = "type")]
    pub stream: ReleaseStream,

    /// The version attached to this release
    pub version: Version,

    /// The size of this file in bytes
    #[serde(rename = "filesize")]
    pub file_size: u128,

    /// CurseForge version metadata
    pub versions: Vec<String>,

    /// Total downloads for this file
    pub downloads: u32,

    /// Upload timestamp
    pub uploaded_at: String,
}

/// Describes a CurseForge project
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {

    /// Project ID
    pub id: u32,

    /// Project title
    pub title: String,

    /// Project summary
    pub summary: String,

    /// The name of the game this project is built for
    pub game: String,

    /// The CurseForge category for this project (mod, modpack, ...)
    #[serde(rename = "type")]
    pub project_type: String,

    /// Urls for this project
    pub urls: HashMap<String, Url>,

    /// Thumbnail image Url
    pub thumbnail: Url,

    /// Project creation timestamp
    pub created_at: String,

    /// Number of downloads for this project
    pub downloads: DownloadsMetric,

    /// Project license
    pub license: String,

    /// Project donation info
    pub donate: String,

    /// Project members
    pub members: Vec<ProjectMember>,

    /// All files attached to the project
    pub files: Vec<ProjectFile>,

    /// All files attached to the project sorted by version
    pub versions: HashMap<Version, Vec<ProjectFile>>,

    /// Project description
    pub description: String,

    /// The timestamp of the last data fetch for this project
    pub last_fetch: String,

    /// The latest file for this project
    #[serde(rename = "download")]
    pub latest_file: ProjectFile
}
