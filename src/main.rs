mod parser;
mod bruteforce;
mod tester;
mod heldkarp;
mod tabu;
mod genetic;

use std::{env, fs, thread};
use std::char::MAX;
use std::fs::{File, OpenOptions};
use std::io::stdin;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Instant;
use itertools::Itertools;
use csv::Writer;
use rand::Rng;
use crate::genetic::pmx;
use crate::heldkarp::held_karp;
use crate::parser::generate_graph;
use crate::tabu::localsearch;


fn auto_test(n: i32, max: i32, count: i32) -> (){
    // Create a CSV file or append to an existing one
    let mut autowriter = Writer::from_path("autoresultsn".to_owned() + &n.to_string() + ".csv").expect("Unable to create/open results.csv");

    for _ in 0..count{
        let graph = parser::generate_graph(n,max);

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

    for i in 0..count {
        let now = Instant::now();
        let (best, path) = genetic::genetic(&graph, 1000);

        let time = now.elapsed().as_micros();
        println!("elapsed: {} us", time);
        println!("best score: {}", best);
        println!("Path: {:?}", path);
        ovtime += time as i32;
        ovbest += best;
    }
    writer.serialize((filepath, ovbest/count, ((ovbest as f32 /count as f32) /opt as f32) ,(ovtime/count) as f32 / 1000000.0)).expect("Failed to write results to CSV");
//path.into_iter().map(|x| x.to_string() + "-").collect::<String>()
}

fn inifile(ini: String) -> (){
    let csvpath = ini.split(" ").last().expect("Empty ini file");

    let mut writer = Writer::from_path("results.csv").expect("Unable to create/open results.csv");
    let mut iniiter = ini.split("\n");

    for term in iniiter {
        if term.clone().split(" ").count() <= 1 {continue;}
        let mut filepath = term.split(" ").nth(0).expect("");
        let mut count = term.split(" ").nth(1).expect("").parse().expect("");
        let mut opt = term.split(" ").nth(2).expect("").parse().expect("");

        initester(filepath, count, opt, &mut writer);
    }
}

fn genetictests() -> (){

    let path1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let path2 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

    let (pchild1, pchild2) = pmx(&path1, &path2);
    let (ochild1, ochild2) = genetic::ox(&path1, &path2);
    println!("Parent 1: {:?}", path1);
    println!("Parent 2: {:?}", path2);
    println!("Child 1:  {:?}", pchild1);
    println!("Child 2:  {:?}", pchild2);
    println!("Child 1:  {:?}", ochild1);
    println!("Child 2:  {:?}", ochild2);

}

fn main() {

    //genetictests();
    //genetic::genetic(&generate_graph(20,100), 200);
    let iniResult = fs::read_to_string("config.ini");
    let ini = match iniResult {
        Ok(file) => file,
        Err(error) =>{
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

/*
let args: Vec<String> = env::args().collect();

if args.len() <= 1 {
    println!("File based usage: ./PEA.exe <filename1> <filename2> (...)");
    println!("Generating graphs mode");
    auto_test(4,1000,10000);
}

// Create a CSV file or append to an existing one
let mut writer = Writer::from_path("results.csv").expect("Unable to create/open results.csv");

for fpath in &args[1..]{
    println!("{}", fpath.clone());
    let file_content = fs::read_to_string(fpath).expect("Should read the file unless an error occurs");

    let graph = parser::parse_graph_matrix(&*file_content);

    /*let now = Instant::now();
    let (best, path) = bruteforce::brute_force_MT(&graph);

    let time = now.elapsed().as_micros();
    println!("elapsed: {} us", time);
    println!("best score: {}", best);
    println!("Path: {:?}", path);

    let now = Instant::now();
    let (best, path) = bruteforce::brute_force(&graph);

    let time = now.elapsed().as_micros();
    println!("elapsed: {} us", time);
    println!("best score: {}", best);
    println!("Path: {:?}", path);
    */
    let now = Instant::now();
    let (best, path) = heldkarp::held_karp(&graph);

    let time = now.elapsed().as_micros();
    println!("elapsed: {} us", time);
    println!("best score: {}", best);
    println!("Path: {:?}", path);

    // Write the results to the CSV file
    //writer.serialize((fpath, best, time, path.into_iter().map(|x| x.to_string() + "-").collect::<String>()))
    //    .expect("Failed to write results to CSV");
}


*/
}
