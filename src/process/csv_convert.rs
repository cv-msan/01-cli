use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs;
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut rdr = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    // println!("{:?}",headers);
    for result in rdr.records() {
        let record = result?.clone();
        //headers.iter是使用headers的迭代器
        //record.iter()record的迭代器
        //zip是将两个迭代器合并成一个元组迭代器[(header,record),...]
        //collect::<Value>将元组迭代器转化为Json Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
        // println!("{:?}", record);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    //写入到文件里
    fs::write(output, content)?;
    Ok(())
}
