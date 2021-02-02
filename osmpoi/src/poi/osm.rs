use futures::{TryFuture, TryStreamExt};
use osmpbfreader::{objects::OsmObj, OsmPbfReader};
use std::fs::File;
use std::path::Path;

/// A struct that represents the counts of different elements in a file.
#[derive(Clone, Copy, Debug)]
pub struct OsmCount {
    pub node: i64,
    pub way: i64,
    pub relation: i64,
}
impl OsmCount {
    /// Create a new count object with all fields set to 0.
    fn new() -> Self {
        Self {
            node: 0,
            way: 0,
            relation: 0,
        }
    }
    /// Increase the count according to the type of obj it reads.
    fn incr(self, obj: OsmObj) -> OsmCount {
        match obj {
            OsmObj::Node(_) => self.node += 1,
            OsmObj::Way(_) => self.way += 1,
            OsmObj::Relation(_) => self.relation += 1,
        }
        self
    }
}

/// count the elements in a osm file
pub fn count_osm<P: AsRef<Path>>(osm_path: P) -> anyhow::Result<OsmCount> {
    Ok(pbf_reader_from_path(osm_path)?.par_iter().try_fold(
        OsmCount::new(),
        |count, obj_result| match obj_result {
            Ok(obj) => Ok(count.incr(obj)),
            Err(err) => Err(err),
        },
    )?)
}

/// dump the osm file with given function, the return value determines whether to continue
pub async fn dump_osm<P, F, Fut>(osm_path: P, dump_fn: F) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(OsmObj) -> Fut,
    Fut: TryFuture<Ok = (), Error = anyhow::Error>,
{
    futures::stream::iter(pbf_reader_from_path(osm_path)?.par_iter())
        .err_into::<anyhow::Error>()
        .try_for_each(dump_fn);
    Ok(())
}

/// create a reader from the path
/// it will check whether the path end with .osm.pbf
fn pbf_reader_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<OsmPbfReader<File>> {
    if !path.as_ref().ends_with(".osm.pbf") {
        return Err(anyhow::Error::msg("The pbf file must end with .osm.pbf"));
    }
    let read = File::open(path)?;
    Ok(OsmPbfReader::new(read))
}

#[cfg(test)]
mod test {
    use super::*;
    use osmpbfreader::objects::*;

    /// whether OsmCount struct add up correctly
    #[test]
    fn osmcount_incr() {
        let count = OsmCount::new();
        let obj = OsmObj::from(new_osm_node());
        assert_eq!(count.node, 1);
    }

    #[test]
    fn get_pbf_from_file() {
        assert!(
            pbf_reader_from_path("name.mp3").is_err(),
            "mp3 file should not be read"
        );
        assert!(
            pbf_reader_from_path("name.osm").is_err(),
            "osm only file should not be read"
        );
        assert!(
            pbf_reader_from_path("name.pbf").is_err(),
            "pbf only file should not be read"
        );
        assert!(
            pbf_reader_from_path("name.osm.pbf").is_ok(),
            "osm.pbf file should be read"
        );
    }

    /// create a new empty osm node
    fn new_osm_node() -> Node {
        Node {
            id: NodeId(0),
            decimicro_lat: 0,
            decimicro_lon: 0,
            tags: Tags::new(),
        }
    }
}
