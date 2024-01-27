mod parser;
mod bruteforce;
mod heldkarp;
mod tabu;
mod genetic;
mod annealing;

use std::{env, fs, thread};
use std::char::MAX;
use std::cmp::min;
use std::f64::consts;
use std::fs::{File, OpenOptions};
use std::io::stdin;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Instant;
use itertools::Itertools;
use csv::Writer;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::parser::generate_graph;


fn auto_test(n: i32, max: i32, count: i32) -> (){
    // Create a CSV file or append to an existing one
    let mut autowriter = Writer::from_path("autoresultsn".to_owned() + &n.to_string() + ".csv").expect("Unable to create/open results.csv");

    for _ in 0..count{
        let graph = generate_graph(n,max);

        let now = Instant::now();
        let (best, path) = heldkarp::held_karp(&graph);

        let time = now.elapsed().as_micros();
        println!("elapsed: {} us", time);
        println!("best score: {}", best);
        println!("Path: {:?}", path);

        // Write the results to the CSV file
        autowriter.serialize((best, time, path.into_iter().map(|x| x.to_string() + ",").collect::<String>()))
            .expect("Failed to write results to CSV");
    }
}

fn initester(filepath: &str, count: i32, opt: i32, writer: &mut Writer<File>) {

    println!("{}", filepath.clone());
    let file_content = fs::read_to_string(filepath).expect("Should read the file unless an error occurs");
    let graph = parser::parse_graph_matrix(&*file_content);

    let mut ovbest: i32 = 0;
    let mut ovtime:i32 = 0;
    let mut realcount: i32 = 0;
    let starttime = Instant::now();
    for _ in 0..count {
        if starttime.elapsed().as_secs() < 300 {
            realcount += 1;
            let now = Instant::now();
            let (best, path) = annealing::annealing(&graph, 0.997);
            let time = now.elapsed().as_micros();
            println!("elapsed: {} us", time);
            println!("best score: {}", best);
            println!("Path: {:?}", path);
            ovtime += time as i32;
            ovbest += best;

            if (realcount + 1) * (now.elapsed().as_secs() as i32) > 300 //proactively disable further action
            { break; }
        }
    }
    writer.serialize((filepath, ovbest/realcount, ((ovbest as f32 /realcount as f32) /opt as f32) ,(ovtime/realcount) as f32 / 1000000.0)).expect("Failed to write results to CSV");
//path.into_iter().map(|x| x.to_string() + "-").collect::<String>()
}

fn inifile(ini: String) -> (){
    let _ = ini.split(" ").last().expect("Empty ini file");

    let mut writer = Writer::from_path("results.csv").expect("Unable to create/open results.csv");
    let iniiter = ini.split("\n");

    for term in iniiter {
        if term.clone().split(" ").count() <= 1 {continue;}
        let filepath = term.split(" ").nth(0).expect("");
        let count = term.split(" ").nth(1).expect("").parse().expect("");
        let opt = term.split(" ").nth(2).expect("").parse().expect("");

        initester(filepath, count, opt, &mut writer);
    }
}

fn main() {
    let ini_result = fs::read_to_string("config.ini");
    let ini = match ini_result {
        Ok(file) => file,
        Err(_) => {
            auto_test(6, 1000, 100);
            auto_test(8, 1000, 100);
            auto_test(10, 1000, 100);
            auto_test(12, 1000, 100);
            auto_test(14, 1000, 100);
            auto_test(16, 1000, 100);
            auto_test(17, 1000, 100);
            auto_test(18, 1000, 25);
            auto_test(19, 1000, 25);
            auto_test(20, 1000, 10);
            auto_test(21, 1000, 10);
            auto_test(22, 1000, 10);
            auto_test(23, 1000, 5);
            "xd".to_string()
        }
    };
    inifile(ini);

}