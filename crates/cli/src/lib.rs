pub mod cli_parser;
pub mod schemas;
pub mod validator;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Validate,
    Seed,
}