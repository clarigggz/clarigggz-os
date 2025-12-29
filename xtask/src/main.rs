use std::process::Command;
use clap::{Parser, Subcommand};
use anyhow::Context;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the OS components
    Build,
    /// Run the OS in the engine
    Run,
    /// Package the OS into a .cg firmware file
    Dist,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => {
            println!("Building Clarigggz OS...");
            build_component("kernel")?;
            build_component("bootloader")?;
            build_component("drivers")?;
            build_component("init")?;
            build_component("vfs")?;
        }
        Commands::Run => {
            println!("Running Clarigggz OS...");
            // Logic to call the engine
        }
        Commands::Dist => {
            println!("Packaging Clarigggz OS into .cg format...");
            // 1. Build all
            // 2. Combine binaries
            // 3. Add header and metadata
        }
    }

    Ok(())
}

fn build_component(name: &str) -> anyhow::Result<()> {
    println!("  Building {}...", name);
    let status = Command::new("cargo")
        .args(["build", "-p", name, "--target", "riscv64gc-unknown-none-elf"])
        .status()
        .context(format!("Failed to build {}", name))?;
    
    if !status.success() {
        anyhow::bail!("{} build failed", name);
    }
    Ok(())
}
