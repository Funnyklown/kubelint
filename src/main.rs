use std::{fs::File, io::Read};
use anyhow::Error;
use yaml_rust2::{YamlEmitter, YamlLoader};


fn main() -> anyhow::Result<(), Error>{  
    unsafe { std::env::set_var("RUST_BACKTRACE", "0") };

    let mut test_file =  File::open("src/test.yaml")?;
    let mut contents = String::new();
    test_file.read_to_string(&mut contents)?;
    let docs = YamlLoader::load_from_str(&contents)?;

    let doc = &docs[0];

    println!("{:?}", doc);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc)?; // dump the YAML object to a String
    }
    println!("{}", out_str);

    Ok(())
}