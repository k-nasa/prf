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

    let remote = matches.value_of("remote").unwrap_or("origin");

    let (owner, repo) = read_gitconfig(&remote)?;
    let owner = matches.value_of("owner").unwrap_or(&owner);
    let repo = matches.value_of("repository").unwrap_or(&repo);

    let pr_no = matches.value_of("pr_no").unwrap();

    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_no
    );

    let pr: FprResult<PullRequest> = async_std::task::block_on(async {
        let res: PullRequest = match surf::get(url.clone()).recv_json().await {
            Err(_) => return Err(anyhow!("feailed fetch pull request.\nfetch url: {}", url).into()),
            Ok(r) => r,
        };

        Ok(res)
    });

    let ref_string = pr?.head.ref_string;

    let arg = format!("pull/{}/head:{}", pr_no, ref_string);
    let output = Command::new("git").args(&["fetch", &remote, &arg]).output();

    let output = match output {
        Ok(output) => output,
        Err(_) => return Err(anyhow!("git command execution failed").into()),
    };

    let stderr = std::str::from_utf8(&output.stderr)?.trim();

    if !stderr.is_empty() {
        return Err(anyhow!("{}", stderr).into());
    }

    println!("create new branch: {}", ref_string);

    Ok(())
}

fn read_gitconfig(remote: &str) -> FprResult<(String, String)> {
    let remote = format!("remote.{}.url", remote);

    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg(remote)
        .output();

    let output = match output {
        Ok(output) => output,
        Err(_) => return Err(anyhow!("git command execution failed").into()),
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
            Arg::with_name("remote")
                .value_name("remote")
                .default_value("origin"),
        )
        .arg(
            Arg::with_name("owner")
                .short("o")
                .long("owner")
                .help("repository owner (By default it uses the local repository's remote url)")
                .value_name("owner"),
        )
        .arg(
            Arg::with_name("repository")
                .short("r")
                .long("repository")
                .help("repository name (By default it uses the local repository's remote url)")
                .value_name("repository"),
        )
}
