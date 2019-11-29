use std::process::Command;

use anyhow::anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{Deserialize, Serialize};

type FprResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {
    pub head: Head,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
    #[serde(rename = "ref")]
    pub ref_string: String,
}

fn main() -> FprResult<()> {
    let app = build_app();
    let matches = app.get_matches();

    let (owner, repo) = read_gitconfig()?;

    let owner = matches.value_of("owner").unwrap_or(&owner);
    let repo = matches.value_of("repository").unwrap_or(&repo);

    let pr_no = matches.value_of("pr_no").unwrap();

    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_no
    );

    let pr: FprResult<PullRequest> = async_std::task::block_on(async {
        let res: PullRequest = match surf::get(url.clone()).recv_json().await {
            Err(_) => Err(anyhow!("feailed fetch pull request.\nfetch url: {}", url))?,
            Ok(r) => r,
        };

        Ok(res)
    });

    let ref_string = pr?.head.ref_string;

    let arg = format!("pull/{}/head:{}", pr_no, ref_string);
    let output = Command::new("git").args(&["fetch", &arg]).output();

    let output = match output {
        Ok(output) => output,
        Err(_) => Err(anyhow!("git command execution failed"))?,
    };

    let stderr = std::str::from_utf8(&output.stderr)?.trim();

    if !stderr.is_empty() {
        Err(anyhow!("{}", stderr))?
    }

    Ok(())
}

fn read_gitconfig() -> FprResult<(String, String)> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output();

    let output = match output {
        Ok(output) => output,
        Err(_) => Err(anyhow!("git command execution failed"))?,
    };

    let origin_url = std::str::from_utf8(&output.stdout)?.trim();

    let owner = origin_url
        .split('/')
        .nth(3)
        .ok_or_else(|| "Reading of origin url failed")?;

    let repo = origin_url
        .split('/')
        .nth(4)
        .ok_or_else(|| "Reading of origin url failed")?
        .trim_end_matches(".git");

    Ok((owner.to_owned(), repo.to_owned()))
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("pr_no")
                .help("pull request number fetching to local")
                .value_name("pr_no")
                .required(true),
        )
        .arg(
            Arg::with_name("owner")
                .short("o")
                .long("owner")
                .help("repository owner (By default it uses the local repository's origin url)")
                .value_name("owner"),
        )
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repository")
                .help("repository name (By default it uses the local repository's origin url)")
                .value_name("repository"),
        )
}
