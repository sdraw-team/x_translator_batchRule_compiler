use std::fs::File;
use std::io::Write;
use std::path::Path;
use clap::Parser;
use serde::Deserialize;
use csv::Reader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI
{
    input:Option<String>,
    output:Option<String>,
}

#[derive(Debug, Deserialize)]
struct TextPair
{
    english: String,
    chinese: String,
}


// 读取CSV文件
fn read_csv_file(path: impl AsRef<Path>) -> Vec<TextPair> {
    let mut rdr = Reader::from_path(path).unwrap();
    let records = rdr.deserialize();
    let mut text_pairs = Vec::new();
    for result in records {
        let record: TextPair = result.unwrap();
        text_pairs.push(record);
    }
    text_pairs.sort_by(|a, b| {
        a.english.len().cmp(&b.english.len())
    });
    text_pairs.reverse();
    text_pairs
}



fn main() {
    let cli = CLI::parse();
    let input_file_name = cli.input.unwrap_or_else(|| "unload.csv".to_string());
    let default_output_file_name = cli.output.unwrap_or_else(|| "output.txt".to_string());
    let text_pairs = read_csv_file(input_file_name);
    let mut writer = File::create(default_output_file_name).unwrap();
    let bom: [u8; 3] = [0xEF, 0xBB, 0xBF];

    // 首先写入BOM
    writer.write_all(&bom).unwrap();
    for text_pair in text_pairs {
        writer.write_all(format!(r#"
StartRule
Search={}
Replace={}
Pattern=%REPLACE% %ORIG%
select=0
mode=0
EndRule
    "#,text_pair.english,text_pair.chinese).as_bytes()).unwrap();
    }
    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_csv_file() {
        let text_pairs = read_csv_file("unload.csv");
        println!("{:?}", text_pairs)
    }
}