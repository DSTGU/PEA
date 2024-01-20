use std::cmp::min;
use priority_queue::PriorityQueue;
use rand::{random, Rng, thread_rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use crate::tabu::cadlen;

pub fn invert(graph: &Vec<Vec<i32>>, path: &Vec<usize>,j:usize ,k:usize) -> (i32, Vec<usize>){
    let mut afterpath = path.clone();
    let mut j = j;
    let mut k = k;
    while j < k{
        afterpath.swap(j,k);
        j += 1;
        k -= 1;
    }
    (calculate_cost(graph,&afterpath),afterpath)
}

pub fn ox(path1: &Vec<usize>, path2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let len = path1.len();
    let mut rng = rand::thread_rng();

    // Choose two random indices for the crossover
    let start = rng.gen_range(0..len);
    let end = rng.gen_range(start + 1..=len);

    // Copy the selected portion from parents to offspring
    let mut child1 = path1[start..end].to_vec();
    let mut child2 = path2[start..end].to_vec();

    // Indices for copying the remaining elements from parent2 to child1 and vice versa
    let mut idx1 = end % len;
    let mut idx2 = end % len;

    // Fill the remaining elements in child1 with values from parent2
    while child1.len() < len {
        if !child1.contains(&path2[idx2]) {
            child1.push(path2[idx2]);
        }
        idx2 = (idx2 + 1) % len;
    }

    // Fill the remaining elements in child2 with values from parent1
    while child2.len() < len {
        if !child2.contains(&path1[idx1]) {
            child2.push(path1[idx1]);
        }
        idx1 = (idx1 + 1) % len;
    }

    (child1, child2)
}

pub fn pmx(path1: &Vec<usize>, path2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let len = path1.len();
    let mut rng = rand::thread_rng();

    // Choose two random indices for the crossover
    let start = rng.gen_range(0..len);
    let end = rng.gen_range(start + 1..=len);

    // Copy the selected portion from parents to offspring
    let mut child1 = path1[start..end].to_vec();
    let mut child2 = path2[start..end].to_vec();

    // Map the values in the selected portion
    let mut mapping = vec![0; len+1];


    for i in 0..child1.len() {
        mapping[child1[i]] = child2[i];
        mapping[child2[i]] = child1[i];
    }

    // Apply mapping to the remaining elements
    for i in 0..len {
        if i < start || i >= end {
            let mut current = path1[i];
            while child1.contains(&current) {
                current = mapping[current];
            }
            child1.push(current);

            let mut current = path2[i];
            while child2.contains(&current) {
                current = mapping[current];
            }
            child2.push(current);
        }
    }

    (child1, child2)
}


fn randompath(graph: &Vec<Vec<i32>>, rng: &mut ThreadRng) -> (i32, Vec<usize>) {

    let mut ver :Vec<usize> = (1..graph.len()).collect();
    ver.shuffle(rng);

    (calculate_cost(graph, &ver), ver)
}

fn calculate_cost(graph: &Vec<Vec<i32>>, path: &Vec<usize>) -> i32
{
    let mut cost = graph[0][path[0]];
    for i in 0..path.len() - 1 {
        cost += graph[path[i]][path[i + 1]];
    }
    cost += graph[path[path.len()-1]][0];
    cost
}

fn mutate(mut path: Vec<usize>, percentage: usize, rng: &mut ThreadRng) -> Vec<usize>{

    let length = path.len();
    if rng.gen_range(0..100) > percentage {
        path.swap(rng.gen_range(0..length), rng.gen_range(0..length))
    }

    path


}

fn create_subpopulation(graph: &Vec<Vec<i32>>, population: Vec<(i32, Vec<usize>)>, maxpopulation: usize) -> Vec<(i32, Vec<usize>)>{

    let mut subpopulation = population.clone();
    subpopulation.truncate(25);
    let mut rng = rand::thread_rng();
    let populationlen = population.len();

    while subpopulation.len() < maxpopulation {
        let index1 = min(min(rng.gen_range(0..populationlen), rng.gen_range(0..populationlen)), min(rng.gen_range(0..populationlen), rng.gen_range(0..populationlen)));
        let index2 = min(min(rng.gen_range(0..populationlen), rng.gen_range(0..populationlen)), min(rng.gen_range(0..populationlen), rng.gen_range(0..populationlen)));

        let mut children = pmx(&population[index1].1, &population[index2].1);
        let mut mutatedchildren0 = mutate(children.0, 2, &mut rng);
        let mut mutatedchildren1 = mutate(children.1, 2, &mut rng);
        subpopulation.push((calculate_cost(graph,&mutatedchildren0), mutatedchildren0));
        subpopulation.push((calculate_cost(graph,&mutatedchildren1), mutatedchildren1));
    }

    subpopulation
}



pub fn genetic(graph: &Vec<Vec<i32>>, maxpopulation: usize) -> (i32, Vec<usize>)
{
    let mut population = vec![];

    let mut rng = thread_rng();
    let mut iteration = 0;

    while population.len() < maxpopulation {
        let randomsol = randompath(&graph, &mut rng);
        population.push(randomsol);
    }

    let mut best = (i32::MAX, vec![]);

    loop {



        population.sort_by(|(a, _), (b, _)| a.cmp(b));

        population.truncate(population.len()/4);
        //println!("{:?}",population[0]);

        if population[0].0 < best.0 {
            best = population[0].clone();
        }

        population = create_subpopulation(graph, population, maxpopulation);


        iteration = iteration + 1;
        if iteration > 1000 {
            break;
        }
    }


    best

}
















