use clap::Parser;

mod func;
mod cli;
mod gui;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct Cli {
    #[arg(long)]
    gui: bool,
    #[arg(short, long)]
    help: bool
}

fn main() {
    let cli = Cli::parse();
    
    match cli.help {
        true => {
            println!("Type --gui for GUI interface");
            return;
        }
        false => {}
    }
    
    match cli.gui {
        true => gui::gui(),
        false => cli::from_config(),
    }
}