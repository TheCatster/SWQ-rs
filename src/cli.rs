use crate::{gif::get_gif, quote::get_quote};
use anyhow::Result;
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version, ArgMatches};
use clipboard_ext::{prelude::*, x11_fork::ClipboardContext};

pub fn run() {
    let matches: ArgMatches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@subcommand quote =>
                (about: "Retrieves Star Wars quotes")
                (@arg KEYWORDS: +required "Sets the keywords to search quotes for")
        )
        (@subcommand gif =>
                (about: "Retrieves Star Wars GIFs")
                (@arg KEYWORDS: +required "Sets the keywords to search GIFs for")
        )
    )
    .get_matches();

    let _result = handle_commands(matches).expect("Unexpected error!");
}

fn handle_commands(matches: ArgMatches) -> Result<(), anyhow::Error> {
    let result = match matches.subcommand() {
        ("quote", Some(sub_m)) => get_quote(sub_m.value_of("KEYWORDS").unwrap()),
        ("gif", Some(sub_m)) => get_gif(sub_m.value_of("KEYWORDS").unwrap()),
        _ => unreachable!(),
    }?;

    println!("{}\n", result);

    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to open clipboard");
    ctx.set_contents(result)
        .expect("Failed to set clipboard contents");

    println!("Added to your clipboard!");

    Ok(())
}
