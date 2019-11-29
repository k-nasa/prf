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
