// Code to deserialise json data in route_guide_db.json
//
// data is a list of the json like the following
// {
//   "location": {
//     "latitude": 407838351,
//     "longitude": -746143763
//   },
//   "name": "Patriots Path, Mendham, NJ 07945, USA"
// },

use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<crate::routeguide::Feature> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "data"]);
    let file = File::open(data_dir.join("route_guide_db.json")).expect("failed to open data file");

    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");
    //decoded json

    //put the list of features into the server
    decoded
        .into_iter()
        .map(|feature| crate::routeguide::Feature {
            name: feature.name,
            location: Some(crate::routeguide::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}
