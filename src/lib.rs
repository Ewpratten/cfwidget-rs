//! `cfwidget` is a crate for fetching data from [cfwidget.com](https://cfwidget.com)
//!
//! This crate contains two main functions for reading data:
//!  - `fetch_project_by_info`
//!  - `fetch_project_by_id`
//!
//! Read their documentation for more info

pub mod types;

use crate::types::Project;
use failure::Error;

/// Fetch a CurseForge project's info by some data describing the project.
///
/// For example, Xaero's Minimap mod for Minecraft would be:
///
/// ```
/// let minimap_mod = fetch_project_by_info("minecraft", "mc-mods", "xaeros-minimap").unwrap();
/// ```
///
/// You can generally get this info from a project's CurseForge Url
pub async fn fetch_project_by_info(
    game_name: &str,
    category: &str,
    project_name: &str,
) -> Result<Project, Error> {
    Ok(reqwest::get(format!(
        "https://api.cfwidget.com/{}/{}/{}",
        game_name, category, project_name
    ))
    .await?
    .json()
    .await?)
}

/// Fetch a CurseForge project's info by its project ID
///
/// For example, Xaero's Minimap mod for Minecraft would be:
///
/// ```
/// let minimap_mod = fetch_project_by_id(263420).unwrap();
/// ```
pub async fn fetch_project_by_id(id: u32) -> Result<Project, Error> {
    Ok(reqwest::get(format!("https://api.cfwidget.com/{}", id))
        .await?
        .json()
        .await?)
}
