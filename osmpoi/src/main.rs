use ::osmpoi::*;
use anyhow::{Context, Result};
use clap::Clap;

/// The cli version of osmpoi, which extract poi information from openstreetmap files.
#[derive(Clap)]
#[clap(version = "0.0.1", author = "Xubai Wang <kindredwekingwang@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Ls(List),
    Add(Add),
    Export(Export),
    Rm(Remove),
    Query(Query),
}

/// list all datasets
#[derive(Clap)]
struct List;

/// add new dataset, only .osm, .osm.pbf, .osm.poi file formats are acceptable
#[derive(Clap)]
struct Add {
    /// the file to add
    path: String,
}

/// export dataset in osm.poi format
#[derive(Clap)]
struct Export {
    /// the dataset to export
    name: String,
    /// where to export
    path: String,
}

/// remove dataset
#[derive(Clap)]
struct Remove {
    /// the file to remove
    name: String,
}

#[derive(Clap)]
struct Query {
    /// the path of the csv file
    path: String,
    /// the name of the dataset
    name: String,
    /// where to output csv
    output_path: String,
    /// how long,
    distance: f32,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Ls(_list) => {
            let items = list_data_dir()?;
            for item in items {
                println!("{}", item);
            }
        }
        SubCommand::Add(add) => {
            let path = data_dir()?;
            std::fs::create_dir_all(path).context("Cannot create subdir")?;
            add_osm_pbf(add.path)?;
        }
        SubCommand::Export(export) => {
            let path = data_dir()?.join(export.name);
            std::fs::copy(path, export.path)?;
        }
        SubCommand::Rm(remove) => {
            let path = data_dir()?.join(remove.name);
            std::fs::remove_file(path)?;
        }
        SubCommand::Query(query) => {
            query_csv(&query.path, &query.output_path, &query.name, query.distance)?;
        }
    }

    Ok(())
}
