use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(name = "goop", about = "Create many guids")]
struct Opt {
    /// Number to create
    #[structopt(default_value = "1")]
    num: usize,
}

fn main() {
    let opt = Opt::from_args();
    (0..opt.num).into_iter().for_each(|_| {
        let uuid = Uuid::new_v4();
        let output = uuid.to_hyphenated().to_string();
        println!("{}", output);
    });
}
