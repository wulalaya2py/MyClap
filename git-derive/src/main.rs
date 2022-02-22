use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser,Subcommand};

/// A fictional versioning CLI2
#[derive(Parser)]
#[clap(name = "git")]
#[clap(about = "A fictional versioning CLI1",long_about=None)]
struct Cli{
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands{
    ///Clones repos
    #[clap(arg_required_else_help = true)]
    Clone {
        ///description for remote
        remote: String,
    },
    ///pushes things
    #[clap(arg_required_else_help = true)]
    Push{
        /// remote for pushing
        remote:String,
    },
    #[clap(arg_required_else_help = true)]
    Add {
        ///Stuff to add
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}
fn main() {
    let args = Cli::parse();
    match &args.command{
        Commands::Clone {remote} => {
            println!("Cloning {}",remote);
        }
        Commands::Push {remote} =>{
            println!("Pushing to {}",remote);
        }
        Commands::Add {path} => {
            println!("Adding {:?}",path);
        }
        Commands::External(args)=>{
            println!("Calling out to {:?} with {:?}",&args[0],&args[1..]);
        }
    }
}
