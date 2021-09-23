#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
enum Cmd {
    #[structopt(
        name = "reaper",
        about = "cargo reaper is a Linux-only cargo subcommand wrapper to cleanly sigkill its \
                 inner process tree.",
        setting(structopt::clap::AppSettings::AllowLeadingHyphen)
    )]
    Reaper {
        #[structopt(parse(try_from_os_str = cstring_from_osstr))]
        program: CString,
        #[structopt(parse(try_from_os_str = cstring_from_osstr), multiple = true)]
        args: Vec<CString>,
    },
}

fn cstring_from_osstr(os: &OsStr) -> Result<CString, OsString> {
    CString::new(os.as_bytes().to_owned()).map_err(|_| os.to_owned())
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let Cmd::Reaper { program, args } = Cmd::from_args();

    let program = match std::env::var_os("CARGO").and_then(|cargo| cstring_from_osstr(&cargo).ok())
    {
        Some(cargo) if program == CString::new("cargo").unwrap() => cargo,
        _ => program,
    };

    lib_grim_reaper::exec_reaper(&program, &args)
}
