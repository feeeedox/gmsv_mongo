use log::{error, info, warn};
use serde::Deserialize;

#[derive(Deserialize)]
struct GitHubTag {
    name: String,
}

pub(crate) fn check_latest_version() -> Result<(), Box<dyn std::error::Error>> {
    let repo_owner = "feeeedox";
    let repo_name = "gmsv_mongo";
    let url = format!("https://api.github.com/repos/{}/{}/tags", repo_owner, repo_name);

    let response = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0")
        .call();

    if let Ok(response) = response {
        let tags: Vec<GitHubTag> = response.into_json()?;

        if let Some(latest_tag) = tags.first() {
            let current_version = env!("CARGO_PKG_VERSION");

            info!("Checking for updates...");

            if latest_tag.name != current_version {
                warn!("You are using version {}, but the latest version is {}.", current_version, latest_tag.name);
            } else {
                info!("You are using the latest version ({}).", current_version);
            }
        } else {
            error!("Failed to get the latest tag.");
        }
    } else {
        error!("Failed to fetch tags from GitHub.");
    }

    Ok(())
}
