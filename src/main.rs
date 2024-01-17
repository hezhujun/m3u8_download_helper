
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use m3u8_rs;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    base_url: String,
}

fn main() {
    let args = Args::parse();

    let filepath = Path::new(&args.file);
    let dirpath = filepath.with_extension("");
    let filename = filepath.file_name().unwrap();
    println!("创建目录 {}", dirpath.to_str().unwrap());
    std::fs::create_dir_all(&dirpath).expect(&format!("创建目录 {} 失败", dirpath.to_str().unwrap()));
    
    let mut file = File::open(filepath).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let mut pl = match m3u8_rs::parse_media_playlist_res(&bytes) {
        Ok(pl) => pl,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut download_list: Vec<(String, String)> = Vec::new();
    for ele in &mut pl.segments {
        let key = &mut ele.key;
        match key {
            Some(key) => {
                let uri = match &key.uri {
                    Some(uri) => {
                        let uri = complete_uri(&uri, &args.base_url);
                        let filename = parse_filename_from_uri(&uri, "", "");
                        download_list.push((uri, filename.to_owned()));
                        Some(filename)
                    },
                    None => Option::None
                };
                key.uri = uri
            },
            None => {}
        }

        let uri = complete_uri(&ele.uri, &args.base_url);
        let filename = parse_filename_from_uri(&uri, "", "");
        let local_path = Path::new("download").join(filename);
        let local_path = local_path.to_str().unwrap().to_owned();
        download_list.push((uri, local_path.to_owned()));
        ele.uri = local_path;
    }

    let local_m3u8_filepath = dirpath.join(filename);
    let mut write_file = File::create(local_m3u8_filepath).expect("error");
    pl.write_to(&mut write_file).unwrap();

    let download_list_filename = format!("{}_download_list.txt", filename.to_str().unwrap());
    let download_list_filepath = dirpath.join(download_list_filename);
    let mut download_list_file = File::create(download_list_filepath).expect("error");
    for (uri, local_uri) in download_list {
        write!(download_list_file, "{}\n  out={}\n", uri, local_uri).expect("error");
    }
}

fn complete_uri(uri: &str, base_url: &str) -> String {
    match Url::parse(uri) {
        Ok(_) => uri.to_owned(),
        Err(_) => {
            let base_url = Url::parse(base_url).expect(&format!("解析 base_url {} 失败", base_url));
            base_url.join(uri).unwrap().to_string()
        },
    }
}

fn parse_filename_from_uri(uri: &str, prefix: &str, suffix: &str) -> String {
    let parsed = Url::parse(uri).expect(&format!("解析 {} 失败", uri));
    let path = Path::new(parsed.path());
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let file_ext = path.extension().unwrap().to_str().unwrap();
    format!("{}{}{}.{}", prefix, file_stem, suffix, file_ext)
}
