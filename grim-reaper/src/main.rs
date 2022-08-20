#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    about = "grim-reaper is a Linux-only wrapper program to cleanly sigkill its inner deep \
             process tree.",
    setting(clap::AppSettings::AllowLeadingHyphen)
)]
struct GrimReaper {
    #[clap(parse(try_from_os_str = cstring_from_osstr))]
    program: CString,
    #[clap(parse(try_from_os_str = cstring_from_osstr), multiple = true)]
    args: Vec<CString>,
}

fn cstring_from_osstr(os: &OsStr) -> anyhow::Result<CString> {
    CString::new(os.as_bytes().to_owned()).map_err(|_| anyhow::anyhow!("{:?}", os))
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let opt = GrimReaper::from_args();

    lib_grim_reaper::exec_reaper(&opt.program, &opt.args)
}
