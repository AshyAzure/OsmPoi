# OsmPoi

A tool for extracting poi information from the opentreemap file.

This repo is in an early stage and now it only dumps distances between "relations".

## Usage

```sh
cargo run --release -- <data.osm.pbf>
```

This will generate two file: `<data.osm.pbf>.tag.json` which contains dumps all relation tags, and `data.osm.pbf.distance.csv` which calculates Haversine distances (in kilometers) between all relations.
