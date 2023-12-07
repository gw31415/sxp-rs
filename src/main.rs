use std::{
    io::{stdout, BufWriter, Write},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

use cairo::{Context, PdfSurface, Rectangle, SvgSurface};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use poppler::Document;
use rsvg::CairoRenderer;
use url::Url;

/// Convert a PDF from/to SVG files using Cairo/Poppler.
#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Extract a single PDF file to SVG files.
    Extract {
        /// The path of the file to extract.
        path: PathBuf,
        /// The prefix string of the output files.
        #[clap(long, default_value = "output")]
        prefix: String,
    },
    /// Merge SVG files into a single PDF.
    Merge {
        /// The paths list of the source files.
        files: Vec<PathBuf>,
    },
    /// Generate shell completions.
    Complete {
        /// The name of the shell.
        #[arg(value_name = "SHELL")]
        completion: Shell,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();
    let mut stdout = BufWriter::new(stdout());
    use SubCommands::*;
    match cli.subcommand {
        Extract { path, prefix } => {
            let doc = Document::from_file(
                Url::from_file_path(path.canonicalize()?).unwrap().as_str(),
                None,
            )?;

            let page_count = doc.n_pages();
            let num_width = (page_count as f64).log10().floor() as usize + 1;
            for i in 0..page_count {
                let page = doc.page(i).unwrap();
                let (width, height) = page.size();
                let name = {
                    // Create filename
                    let mut name = prefix.clone();
                    name.reserve(prefix.len() + 5 + num_width);
                    name.push('-');
                    let label = (i + 1).to_string();
                    for _ in 0..(num_width - label.len()) {
                        name.push('0');
                    }
                    name.push_str(&label);
                    name.push_str(".svg");
                    name
                };

                // Create an SVG surface for each page
                let surface = SvgSurface::new(width, height, Some(name.clone()))?;
                let context = Context::new(&surface)?;

                // Render the page
                page.render_for_printing(&context);

                // Finish the surface to write the SVG file
                context.show_page()?;
                writeln!(stdout, "{name}")?;
                stdout.flush()?;
            }
        }
        Merge { mut files } => {
            let target_file = files.pop().unwrap();
            let source_files = files;

            use rsvg::Loader;
            let surface = PdfSurface::new(0.0, 0.0, target_file)?;
            let context = Context::new(&surface)?;

            for svg_path in source_files {
                // Load the SVG file
                let svg = Loader::new().read_path(svg_path.clone())?;
                let svg_renderer = CairoRenderer::new(&svg);

                // Get the page-size
                let (width, height) = svg_renderer.intrinsic_size_in_pixels().unwrap_or_default();
                surface.set_size(width, height)?;

                // Render the SVG file onto the PDF surface
                svg_renderer.render_document(&context, &Rectangle::new(0.0, 0.0, width, height))?;

                // Finish the current page
                context.show_page()?;

                // Print the processed SVG file.
                stdout.write_all(svg_path.as_os_str().as_bytes())?;
                stdout.write_all(b"\n")?;
                stdout.flush()?;
            }
            surface.finish();
        }
        Complete { completion } => {
            shell_completion(completion);
        }
    }

    Ok(())
}

#[cold]
fn shell_completion(shell: Shell) {
    let mut stdout = BufWriter::new(stdout());
    let mut cmd = Args::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut stdout);
}
