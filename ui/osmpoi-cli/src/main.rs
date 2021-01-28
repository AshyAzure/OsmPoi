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

/// input the csv and get the output
/// input csv
#[derive(Clap)]
struct Query {
    /// the path of the csv file
    input_path: String,
    /// the name of the dataset
    dataset_path: String,
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
            // create data directory
            std::fs::create_dir_all(path).context("Cannot create subdir")?;
            add_osm_pbf(
                &add.path,
                data_dir()?.to_str().context("Cannot convert to string")?,
            )?;
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
            query_csv(
                &query.input_path,
                &query.output_path,
                &query.dataset_path,
                query.distance,
                true,
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
