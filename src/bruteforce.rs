use std::sync::{Arc, Mutex};
use std::thread;
use itertools::Itertools;

fn calculate_cost(graph: &Vec<Vec<i32>>, path: &Vec<&usize>) -> i32
{
    let mut cost = graph[0][*path[0]];
    for i in 0..path.len() - 1 {
        cost += graph[*path[i]][*path[i + 1]];
    }
    cost += graph[*path[path.len()-1]][0];
    cost
}

fn calculate_cost_slice(graph: &[&[i32]], path: &Vec<&usize>) -> i32
{
    let mut cost = graph[0][*path[0]];
    for i in 0..path.len() - 1 {
        cost += graph[*path[i]][*path[i + 1]];
    }
    cost += graph[path.len()][0];
    cost
}

fn calculate_cost_cutoff(graph: &Vec<Vec<i32>>, path: &Vec<&usize>, cutoff: i32) -> i32
{
    let mut cost = graph[0][*path[0]];
    for i in 0..path.len() - 1 {
        let from = *path[i];
        let to = *path[i + 1];
        cost += graph[from][to];
        if cost > cutoff
        {
            return i32::MAX;
        }
    }
    cost += graph[*path[path.len()-1]][0];
    cost
}

fn calculate_cost_MT(graph: &Vec<Vec<i32>>, path: &Vec<&usize>, nr: usize) -> i32 {
    let mut cost = graph[0][nr];
    cost += graph[nr][*path[0]];
    for i in 0..path.len() - 1 {
        let from = *path[i];
        let to = *path[i + 1];
        cost += graph[from][to];
    }
    cost += graph[path.len() + 1][0];
    cost
}

fn brute_force_thread(graph: Vec<Vec<i32>>, nr: usize) -> (i32, Vec<usize>) {
    let size = graph.len();
    let mut best_cost = i32::MAX;
    let mut best_path: Vec<&usize> = Vec::new();
    let mut current_path: Vec<usize> = (1..size).collect();
    current_path.remove(nr-1);


    for perm in current_path.iter().permutations(size-2) { //Generacja wszystkich permutacji za pomocą iteratora
        let current_cost = calculate_cost_MT(&graph, &perm, nr); //Liczenie funkcji celu

        if current_cost < best_cost { //Jeśli obecna permutacja jest lepsza niż wszystkie poprzednie to ją zapisujemy
            best_cost = current_cost;
            best_path = perm;
        }
    }

    let mut best_path_de: Vec<usize> = vec![nr];
    let mut best_path_part = best_path.iter().map(|&x| x.clone()).collect();
    best_path_de.append(&mut best_path_part);

    (best_cost, best_path_de)


}

pub fn brute_force_MT(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    // Define the number of threads and an empty vector to store results
    let num_threads = graph.len()-2; // Change this to the desired number of threads
    let mut results = Vec::with_capacity(num_threads);
    // Create an Arc and a Mutex to share the results between threads
    let shared_results = Arc::new(Mutex::new(results));

    // Spawn threads to execute BFThreadable
    let handles: Vec<_> = (1..graph.len()).map(|nr| {
        let thread_shared_results = shared_results.clone();
        let gclone = graph.clone();
        thread::spawn(move || {
            let result = brute_force_thread(gclone, nr);
            let mut data = thread_shared_results.lock().unwrap();
            data.push(result);
        })
    }).collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Retrieve and print all the results
    let results = shared_results.lock().unwrap();
    let mut l = i32::MAX;
    let mut p = vec![];

    for (i, v) in results.iter().enumerate() {
        if v.0 < l
        {
            l = v.0;
            p = v.1.clone();
        }
    }

    (l,p)
}

//Algorytm
pub fn brute_force(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){

    let size = graph.len();
    let mut best_cost = i32::MAX;
    let mut best_path: Vec<&usize> = Vec::new();

    let mut current_path: Vec<usize> = (1..size).collect();

    for perm in current_path.iter().permutations(size-1) { //Generacja wszystkich permutacji za pomocą iteratora

        let current_cost = calculate_cost(&graph, &perm); //Liczenie funkcji celu
        if current_cost < best_cost { //Jeśli obecna permutacja jest lepsza niż wszystkie poprzednie to ją zapisujemy
            best_cost = current_cost;
            best_path = perm;
        }
    }

    let best_path_de: Vec<usize> = best_path.iter().map(|&x| x.clone()).collect();

    (best_cost, best_path_de)
}

//Algorytm
pub fn brute_force_cutoff(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){
    let size = graph.len();
    let mut best_cost = i32::MAX;
    let mut best_path: Vec<&usize> = Vec::new();

    let mut current_path: Vec<usize> = (1..size).collect();

    for perm in current_path.iter().permutations(size-1) { //Generacja wszystkich permutacji za pomocą iteratora

        let current_cost = calculate_cost_cutoff(&graph, &perm,best_cost); //Liczenie funkcji celu
        if current_cost < best_cost { //Jeśli obecna permutacja jest lepsza niż wszystkie poprzednie to ją zapisujemy
            best_cost = current_cost;
            best_path = perm;
        }
    }

    let best_path_de: Vec<usize> = best_path.iter().map(|&x| x.clone()).collect();

    (best_cost, best_path_de)
}