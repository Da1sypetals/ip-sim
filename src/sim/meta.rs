use super::body::affinebody::AffineBody;
use super::body::body::GenericBody;
use super::body::springsbody::SpringsBody;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn get_bodies() -> Vec<GenericBody> {
    let file_path = "bodies/meta"; // 替换为你的文件路径
    let file = File::open(file_path).expect("Failed to read meta file!");
    let reader = BufReader::new(file);

    let mut res = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect(format!("Failed to read line {i}!").as_str());
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(tag_end) = line.find(' ') {
            let tag = &line[0..tag_end];
            let filename = &line[tag_end + 1..];
            let body = match tag {
                "!affine" => GenericBody::Affine(AffineBody::from_file(filename).expect(
                    format!("Failed to parse an affine body from file [{filename}]").as_str(),
                )),
                "!springs" => {
                    GenericBody::Springs(SpringsBody::from_file_with_v0(filename).expect(
                        format!("Failed to parse an affine body from file [{filename}]").as_str(),
                    ))
                }
                _ => panic!("Unsupported body type: {tag}"),
            };

            res.push(body);
        }
    }

    res
}
