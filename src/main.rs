mod log;
mod net;
mod pckg;
mod proton;
mod steam;

use clap::{App, Arg, ArgMatches, SubCommand};
use steam::Steam;

fn matches_argument() -> ArgMatches<'static> {
    App::new("og")
        .version("0.0.1")
        .author("RomualdAuc")
        .about("A simple program to install gaming dependencies on linux computer")
        .subcommand(
            SubCommand::with_name("proton")
                .about("ProtonGE gestion (Install / Remove / List)")
                .arg(
                    Arg::with_name("install")
                        .short("i")
                        .long("install")
                        .help("Install a specific version of ProtonGE")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("update")
                        .short("u")
                        .long("update")
                        .help("Install latest ProtonGE version")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("archive")
                        .short("a")
                        .long("archive")
                        .help("Install archive with the path")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("Print all version of proton install")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("remove")
                        .short("r")
                        .long("remove")
                        .help("Remove a specific version")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("clean")
                        .short("c")
                        .long("clean")
                        .help("remove the cache dir of protonge")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("gaming")
                .about("Install gaming dependencies")
                .arg(
                    Arg::with_name("lutris")
                        .short("l")
                        .long("lutris")
                        .help("Install lutris package")
                        .takes_value(false),
                ),
        )
        .get_matches()
}

fn main() {
    match Steam::new() {
        Ok(steam) => {
            let matches = matches_argument();

            if let Some(matches) = matches.subcommand_matches("proton") {
                if matches.is_present("install") {
                    proton::install_version(matches.value_of("install").unwrap(), &steam);
                }
                if matches.is_present("remove") {
                    proton::remove_version(matches.value_of("remove").unwrap(), &steam);
                }
                if matches.is_present("list") {
                    proton::list_version(&steam);
                }
                if matches.is_present("archive") {
                    proton::install_archive_version(matches.value_of("archive").unwrap(), &steam);
                }
                if matches.is_present("update") {
                    proton::update_protonge(&steam);
                }
                if matches.is_present("clean") {
                    proton::remove_cache();
                }
            }

            if let Some(matches) = matches.subcommand_matches("gaming") {
                let installer = pckg::find_installer();
                match installer {
                    Ok(n) => n.installer.install_dependencies(n.commands),
                    Err(err) => log::error(err),
                }

                if matches.args.len() == 0 {
                    log::log("Start full installationé");
                } else {
                }
            }
        }
        Err(e) => log::error(e),
    }
}
