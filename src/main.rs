/*
    open a file
    count the incidences of each character
*/

use rayon::prelude::*;
use std::sync::mpsc::channel;

use reqwest;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
// use std::slice::Chunks;

#[macro_use]
extern crate timeit;

use error_chain::error_chain;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn download_file(url: &str, path: &Path) {
    let response = reqwest::blocking::get(url).expect("request failed");
    let text = response.text().expect("body invalid");
    let mut file = File::create(path).expect("failed to create file");
    io::copy(&mut text.as_bytes(), &mut file).expect("failed to copy content");
}

fn char_count(str: &String) -> HashMap<char, i32> {
    let mut counts = HashMap::new();
    for c in str.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    counts
}

fn par_char_count(str: &String, chunk_size: Option<usize>) -> HashMap<char, i32> {
    let chunk_size = chunk_size.unwrap_or(1024);

    // divide input string into chunks, count letters in threads
    let (sender, receiver) = channel();
    let chunkers = str.as_bytes().chunks(chunk_size).collect::<Vec<&[u8]>>();
    chunkers.into_par_iter().for_each_with(sender, |s, chunk| {
        let mut counts = HashMap::new();
        for c in chunk.iter().map(|&c| c as char) {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }
        s.send(counts).unwrap();
    });

    // merge the hashmaps of letter counts
    let res: Vec<_> = receiver.iter().collect();
    let mut counts = HashMap::new();
    res.iter().for_each(|x| {
        x.iter().for_each(|(k, v)| {
            let count = counts.entry(*k).or_insert(0);
            *count += *v;
        })
    });
    counts
}

fn main() {
    // let file_name = ".mobile_dong.txt";
    let file_name = ".random.txt";
    let url = "https://www.gutenberg.org/files/2701/old/moby10b.txt";

    let path = Path::new(file_name);
    if !path.exists() {
        download_file(url, path);
    }

    let mut file = match File::open(path) {
        Err(_why) => panic!("couldn't open {}", file_name),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }

    // timeit!({
    //     char_count(&contents);
    // });

    let mut r = HashMap::new();
    timeit!({
        r = par_char_count(&contents, Some(1024 * 1024));
    });
    println!("{:?}", r);

    // let r = char_count(&contents);
    // println!("{:?}", r);

    // let r = par_char_count(&contents, Some(1024));
    // println!("{:?}", r);
}
