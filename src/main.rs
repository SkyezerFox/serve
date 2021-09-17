use ansi_term::Colour::{Red, White};
use std::net::Ipv4Addr;

use clap::Clap;
use clipboard::{ClipboardContext, ClipboardProvider};
use local_ip_address::local_ip;
use log::error;

#[derive(Clap)]
#[clap(version = "1.0", author = "skyezerfox (Skye) <actuallyori@gmail.com>")]
struct Opts {
    /// Enable debug mode
    #[clap(short, long)]
    debug: bool,
    // The port to start the server on
    #[clap(short, long, default_value = "3000")]
    port: u16,
    // The path to files to serve
    #[clap(default_value = ".")]
    path: String,
    // The host to bind to
    #[clap(long, default_value = "127.0.0.1")]
    host: Ipv4Addr,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    pretty_env_logger::formatted_builder()
        .filter_module("serve", log::LevelFilter::Info)
        .init();
    // clear terminal
    print!("{}[2J", 27 as char);
    // create server bind
    let res =
        warp::serve(warp::fs::dir(opts.path.clone())).try_bind_ephemeral((opts.host, opts.port));
    // test to see if we can serve
    match res {
        Ok((_, future)) => {
            // print splash
            println!("{}", Red.paint(format!(" ┌{:─<48}┐", "")));
            println!("{}", Red.paint(format!(" │{:<48}│", "")));
            println!(" │{:^48}│", "Server started!");
            println!("{}", Red.paint(format!(" │{:<48}│", "")));
            println!(
                " │{:<48}│",
                format!(" - Local: http://{}:{}", opts.host, opts.port)
            );
            // print network ip address if we can
            if let Ok(ip) = local_ip() {
                println!(
                    " │{:<48}│",
                    format!(" - Network: http://{}:{}", ip, opts.port)
                );
            }
            println!(" │ - Serving: {:<36}│", opts.path);
            println!("{}", Red.paint(format!(" │{:<48}│", "")));
            // copy server address to clipboard if we can
            if let Ok::<ClipboardContext, _>(mut ctx) = ClipboardProvider::new() {
                if let Ok(()) = ctx.set_contents(format!("http://{}:{}", opts.host, opts.port)) {
                    println!(" │{:^48}│", "Local address copied to clipboard!");
                    println!("{}", Red.paint(format!(" │{:<48}│", "")));
                }
            }
            println!("{}", Red.paint(format!(" └{:─<48}┘", "")));
            // await for the future to complete
            future.await;
        }
        Err(e) => {
            println!("{}", Red.paint(format!(" ┌{:─<48}┐", "")));
            println!("{}", Red.paint(format!(" │{:<48}│", "")));
            println!(" │{:^48}│", "Server failed to start!");
            println!("{}", Red.paint(format!(" │{:<48}│", "")));
            println!("{}\n", Red.paint(format!(" └{:─<48}┘", "")));
            error!("{}\n", e);
        }
    }
}
