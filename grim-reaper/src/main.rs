#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::prelude::{OsStrExt, OsStringExt},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    about = "grim-reaper is a Linux-only wrapper program to cleanly sigkill its inner deep \
             process tree."
)]
struct GrimReaper {
    #[arg(value_parser = CStringValueParser)]
    program: CString,
    #[arg(allow_hyphen_values = true)]
    #[arg(value_parser = CStringValueParser)]
    args: Vec<CString>,
}

#[derive(Clone)]
struct CStringValueParser;

impl clap::builder::TypedValueParser for CStringValueParser {
    type Value = CString;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        CString::new(value.as_bytes().to_owned())
            .map_err(|err| clap::Error::raw(clap::error::ErrorKind::ValueValidation, err))
    }

    fn parse(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: OsString,
    ) -> Result<Self::Value, clap::Error> {
        CString::new(value.into_vec())
            .map_err(|err| clap::Error::raw(clap::error::ErrorKind::ValueValidation, err))
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let opt = GrimReaper::parse();

    lib_grim_reaper::exec_reaper(&opt.program, &opt.args)
}
