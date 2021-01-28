use anyhow::{Context, Result};
use clap::Clap;
use osmpoi::*;
use std::path::PathBuf;

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

/// add new dataset to your data directory, only .osm.pbf, .osm.poi file formats are acceptable
#[derive(Clap)]
struct Add {
    /// the source of the dataset
    path: String,
}

/// export dataset from your data directory
#[derive(Clap)]
struct Export {
    /// the dataset to export
    name: String,
    /// where to export
    path: String,
}

/// remove dataset from your data directory
#[derive(Clap)]
struct Remove {
    /// the dataset to remove
    name: String,
}

/// query poi information from the dataset
#[derive(Clap)]
struct Query {
    /// the name of the dataset
    dataset: String,
    /// the path of the csv file
    input: String,
    /// where to output csv
    output: String,
    /// the longest distance
    distance: f32,
    /// to use strict mode or not
    #[clap(short)]
    strict: bool,
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
            // create data directory
            std::fs::create_dir_all(&path).context("Cannot create subdir")?;
            let dataset_path = path
                .join(
                    PathBuf::from(&add.path)
                        .file_stem()
                        .context("Cannot get file stem from input")?,
                )
                .with_extension("osm.pbf");
            let dataset_path = dataset_path
                .to_str()
                .context("Cannot convert file to string")?;
            add_osm_pbf(&add.path, dataset_path)?;
        }
        SubCommand::Export(export) => {
            let path = data_dir()?.join(export.name);
            std::fs::copy(path, export.path)?;
        }
        SubCommand::Rm(remove) => {
            let dataset_path = data_dir()?.join(remove.name);
            std::fs::remove_file(dataset_path)?;
        }
        SubCommand::Query(query) => {
            let dataset_path = data_dir()?.join(query.dataset);
            let dataset_path = dataset_path
                .to_str()
                .context("Caonnot convert dataset path to string")?;
            query_csv(
                &query.input,
                &query.output,
                &dataset_path,
                query.distance,
                query.strict,
            )?;
        }
    }

    Ok(())
}

/// get the data dir for the current system
fn data_dir() -> Result<PathBuf> {
    // create data_dirs
    let mut path = dirs::data_dir().context("Cannot open application support folder")?;
    path.push("osmpoi");
    Ok(path)
}

fn list_data_dir() -> Result<Vec<String>> {
    let mut ret = Vec::new();
    for entry in std::fs::read_dir(data_dir()?)? {
        let entry = entry?;
        ret.push(
            entry
                .file_name()
                .into_string()
                .or(Err(anyhow::Error::msg("Cannot list dir")))?,
        )
    }
    Ok(ret)
}
