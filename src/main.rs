use imagecli::{documentation::generate_guide, process};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Enable verbose logging.
    #[structopt(short, long)]
    verbose: bool,

    /// Input files or glob patterns.
    #[structopt(short, long)]
    input: Vec<String>,

    /// Output files.
    #[structopt(short, long)]
    output: Vec<String>,

    /// Image processing pipeline to apply.
    #[structopt(short, long)]
    pipeline: Option<String>,

    /// Ignore all the other flags and regenerate a user guide instead.
    #[structopt(long)]
    generate_guide: bool,

    /// Performs transformation on all images in a folder and output them in another folder.
    #[structopt(long)]
    bulk_mode: bool,
}

fn main() {
    let opt = Opt::from_args();

    let result = if opt.generate_guide {
        generate_guide(opt.verbose)
    } else if opt.bulk_mode {
        Ok(())
    } else {
        process(&opt.input, &opt.output, opt.pipeline, opt.verbose)
    };

    match result {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
