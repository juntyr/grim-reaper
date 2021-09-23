#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "grim-reaper is a Linux-only wrapper program to cleanly sigkill its inner deep \
             process tree.",
    setting(structopt::clap::AppSettings::AllowLeadingHyphen)
)]
struct GrimReaper {
    #[structopt(parse(try_from_os_str = cstring_from_osstr))]
    program: CString,
    #[structopt(parse(try_from_os_str = cstring_from_osstr), multiple = true)]
    args: Vec<CString>,
}

fn cstring_from_osstr(os: &OsStr) -> Result<CString, OsString> {
    CString::new(os.as_bytes().to_owned()).map_err(|_| os.to_owned())
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let opt = GrimReaper::from_args();

    lib_grim_reaper::exec_reaper(&opt.program, &opt.args)
}
