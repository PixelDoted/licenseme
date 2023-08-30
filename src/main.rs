use std::io::stdin;

use clap::Parser;

mod cli;
mod spdx;
mod util;
mod viewers;

fn main() {
    let args = cli::Args::parse();

    // Find Licenses
    let list = spdx::load_list().expect("Failed to load 'SPDX' List");
    let find = list.find(&args.spdx);
    let spdx_id = match find {
        spdx::SPDXFind::Exact(id) => id.to_string(),
        spdx::SPDXFind::Closest(ids) => {
            let s = viewers::select(ids);
            if s.is_none() {
                eprintln!("Cancelled");
                return;
            }

            s.unwrap()
        }
    };

    // Read License from File or Download one
    let license = spdx::load_license(&spdx_id).expect("Failed to find license");

    // Update the license's text
    let text = if license.contains("<copyright holders>") {
        let holder = match args.holder {
            Some(holder) => holder,
            None => {
                println!("\ncopyright holder:");
                let mut holder = String::new();
                stdin().read_line(&mut holder).unwrap();
                holder
            }
        };

        license.replace("<copyright holders>", &holder)
    } else {
        license.to_string()
    }
    .replace("<year>", &args.year);

    // Confirm the license with the user
    if !viewers::license(&text) {
        println!("Cancelled");
        return;
    }

    std::fs::write(args.path, text).expect("Failed to write license.");
}
