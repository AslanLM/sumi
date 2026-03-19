use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(required = true, trailing_var_arg = true)]
    pub command: Vec<String>,
}
