#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
enum Cmd {
    #[clap(
        name = "reaper",
        about = "cargo reaper is a Linux-only cargo subcommand wrapper to cleanly sigkill its \
                 inner process tree.",
        setting(clap::AppSettings::AllowLeadingHyphen)
    )]
    Reaper {
        #[clap(parse(try_from_os_str = cstring_from_osstr))]
        program: CString,
        #[clap(parse(try_from_os_str = cstring_from_osstr), multiple = true)]
        args: Vec<CString>,
    },
}

fn cstring_from_osstr(os: &OsStr) -> anyhow::Result<CString> {
    CString::new(os.as_bytes().to_owned()).map_err(|_| anyhow::anyhow!("{:?}", os))
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
