use clap::Parser;

mod func;
mod gui;

#[derive(Parser, Debug)]
#[command(about)]
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
        false => func::func::from_config(),
    }
}