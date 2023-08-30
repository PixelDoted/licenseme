use clap::Parser;

#[derive(Parser)]
#[command()]
/// Add a license to your project
pub struct Args {
    /// The SPDX id of a license
    pub spdx: String,

    #[arg(long)]
    /// The copyright holder
    pub holder: Option<String>,

    #[arg(long, default_value_t = year())]
    /// The copyright year
    pub year: String,

    #[arg(short, long, default_value = "./LICENSE")]
    /// The License's path
    pub path: String,
}

fn year() -> String {
    let now = chrono::Local::now();
    now.format("%Y").to_string()
}
