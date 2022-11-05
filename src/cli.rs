use anyhow::Result;
use clap::{Parser, Subcommand};

/// Diff two http requests and compare the difference of the responses
///
/// 对第三方没有用的，实现为 crate 内部的
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about=None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

pub(crate) struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args. Could be used to override the query, headers and body of the request
    /// For query params, use `-e key=value`.
    /// For headers, use `-e %key=value`.
    /// For body, use `-e @key=value`
    #[clap(short, long, value_parser=parse_key_val, number_of_values=1)]
    extra_params: Vec<KeyVal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyValType {
    Querying,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeyVal {
    pub key_type: KeyValType,
    pub key: String,
    pub value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
        .trim();
    let val = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        v if char::is_ascii_alphabetic(&v) => (KeyValType::Querying, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: val.to_string(),
    })
}
