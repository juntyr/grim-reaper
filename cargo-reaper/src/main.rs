#![deny(clippy::pedantic)]

use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::prelude::{OsStrExt, OsStringExt},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
enum Cmd {
    #[clap(
        name = "reaper",
        about = "cargo reaper is a Linux-only cargo subcommand wrapper to cleanly sigkill its \
                 inner process tree."
    )]
    Reaper {
        #[arg(value_parser = CStringValueParser)]
        program: CString,
        #[arg(allow_hyphen_values = true)]
        #[arg(value_parser = CStringValueParser)]
        args: Vec<CString>,
    },
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

    let Cmd::Reaper { program, args } = Cmd::parse();

    let program =
        match std::env::var_os("CARGO").and_then(|cargo| CString::new(cargo.into_vec()).ok()) {
            Some(cargo) if program == CString::new("cargo").unwrap() => cargo,
            _ => program,
        };

    lib_grim_reaper::exec_reaper(&program, &args)
}
