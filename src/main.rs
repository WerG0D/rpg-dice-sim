use clap::{ArgAction, Parser};
use rpg_dice_sim::{compute_stats, AdvMode, Expression};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// expressão tipo 2d6+3, 3d6+2d8-1, d20+5
    expr: String,

    /// repete a rolagem n vezes e mostra estatisticas
    #[arg(short='t', long="times", default_value_t = 1)]
    times: usize,

    /// vantagem (só no d20 “solto”)
    #[arg(long="adv", conflicts_with="dis")]
    adv: bool,

    /// desvantagem
    #[arg(long="dis", conflicts_with="adv")]
    dis: bool,

    /// só o total blá blá blá
    #[arg(long="quiet", short='q', action=ArgAction::SetTrue)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();

    let adv_mode = if cli.adv {
        AdvMode::Advantage
    } else if cli.dis {
        AdvMode::Disadvantage
    } else {
        AdvMode::None
    };

    let expr = match Expression::parse(&cli.expr) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("erro na expressão: {e}");
            std::process::exit(1);
        }
    };

    let mut totals = Vec::new();

    for i in 1..=cli.times {
        let result = expr.roll(adv_mode);

        if !cli.quiet {
            println!("--- roll {i} ---");
            for d in &result.details {
                println!("{d}");
            }
            if result.flat_total != 0 {
                println!("mods: {}", result.flat_total);
            }
        }

        println!("total: {}", result.total);
        if !cli.quiet && i != cli.times {
            println!();
        }
        totals.push(result.total);
    }

    if cli.times > 1 {
        if let Some(stats) = compute_stats(&totals) {
            println!("\n=== estatisticas ===");
            println!("rolagens: {}", stats.count);
            println!("minimo:   {}", stats.min);
            println!("maximo:   {}", stats.max);
            println!("media:    {:.2}", stats.mean);
        }
    }
}
