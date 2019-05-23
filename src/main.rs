use std::io;
use html2sauron::Opt;
use structopt::StructOpt;


fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let html = html2sauron::read_file(&opt.file)?;
    let sauron = html2sauron::html2sauron(&html, &opt);
    if let Some(output) = &opt.output {
        html2sauron::write_to_file(output, &sauron)?;
    } else {
        println!("{}", sauron);
    }
    Ok(())
}
