//! Command-line argument parser
//!
//! Useful links:
//! <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
//!
//! <https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html>
//!
//! <https://github.com/clap-rs/clap/tree/master/examples>

mod args;
mod book_toml;
pub(crate) mod config;
pub(crate) mod links_commands;
pub(crate) mod markdown_commands;
pub(crate) mod refdefs_commands;

use args::*;
use clap::Parser;
use clap::Subcommand;
use links_commands::LinksSubCommand;
use markdown_commands::MarkdownSubCommand;
use refdefs_commands::RefDefsSubCommand;

/// Parse command-line arguments
pub(crate) fn parse_arguments() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
// Reads the following attributes from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays the help, if no arguments are provided
// #[command(arg_required_else_help = true)]
/// Command-line arguments
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    // This structure allows the addition of global options, if needed
    //#[clap(flatten)]
    // global_opts: GlobalOpts,
}

/// Command-line commands
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Manage reference definitions
    #[command(subcommand, name = "refdefs")]
    RefDefs(RefDefsSubCommand),

    /// Manage links
    #[command(subcommand)]
    Links(LinksSubCommand),

    /// Manage code blocks (embedded examples) and includes
    #[command(subcommand)]
    Markdown(MarkdownSubCommand),

    /// Generate a sitemap.xml file from the list of Markdown files
    /// in a source directory
    #[command(name = "sitemap")]
    SiteMap(MarkdownSrcDirUrlAndDestFileArgs),

    /// Parse the entire Markdown code as events
    /// and write them to a file.
    Debug(MarkdownSrcDirAndDestFileArgs),

    /// Test Markdown parsing
    #[allow(dead_code)]
    #[command(skip)]
    Test,
}

// // Example global args, if needed

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     /// Color
//     #[clap(long, arg_enum, global = true, default_value_t =
// Color::Auto)]     color: Color,

//     /// Verbosity level (can be specified multiple times)
//     #[clap(long, short, global = true, parse(from_occurrences))]
//     verbose: usize,
//     //... other global options
// }

// #[derive(Clone, Debug, ArgEnum)]
// enum Color {
//     Always,
//     Auto,
//     Never,
// }
