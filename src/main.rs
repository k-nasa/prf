use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = build_app();

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("help", Some(_)) | _ => app.print_help()?,
    }

    Ok(())
}

fn read_gitconfig() -> FprResult<(String, String)> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .expect("Failed run git config command");

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
                .value_name("pr_no"),
        )
        .subcommand(
            SubCommand::with_name("help")
                .alias("h")
                .about("Show help")
                .setting(AppSettings::ColoredHelp),
        )
}
