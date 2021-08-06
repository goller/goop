use structopt::StructOpt;
use uuid::Uuid;

use std::io::Write;

#[derive(Debug, StructOpt)]
#[structopt(name = "goop", about = "Create many guids")]
struct Opt {
    /// Number to create
    #[structopt(default_value = "1")]
    num: usize,
}

fn main() {
    let opt = Opt::from_args();

    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    let mut buf = Uuid::encode_buffer();

    (0..opt.num).into_iter().for_each(|_| {
        let uuid = Uuid::new_v4();
        let output = uuid.to_hyphenated_ref().encode_lower(&mut buf);
        writeln!(&mut stdout, "{}", output).unwrap();
    });
}
