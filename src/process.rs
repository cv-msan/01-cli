use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let record: Player = result?;
        ret.push(record);
        // println!("{:?}", record);
    }
    //生成jsonString
    let json = serde_json::to_string(&ret)?;
    //写入到文件里
    std::fs::write(output, json)?;
    Ok(())
}
