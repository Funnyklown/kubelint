use std::{fs::File, io::Read};
use anyhow::Error;
use yaml_rust2::{YamlEmitter, YamlLoader};
use clap::Parser;

/// Small cli that lints kubernetes config files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .yaml
    #[arg(short, long)]
    file: String,

}

fn main() -> anyhow::Result<(), Error>{  
    unsafe { std::env::set_var("RUST_BACKTRACE", "0") };

    let args = Args::parse();
    let mut test_file =  File::open(args.file)?;
    let mut contents = String::new();
    test_file.read_to_string(&mut contents)?;
    let docs = YamlLoader::load_from_str(&contents)?;

    let doc = &docs[0];

    println!("{:?}", doc);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc)?;
    }
    println!("{}", out_str);

    Ok(())
}