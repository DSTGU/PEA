use priority_queue::PriorityQueue;
use rand::rngs::ThreadRng;
use rand::{random, thread_rng};
use rand::prelude::SliceRandom;

fn calculate_cost(graph: &Vec<Vec<i32>>, path: &Vec<usize>) -> i32
{
    let mut cost = graph[0][path[0]];
    for i in 0..path.len() - 1 {
        cost += graph[path[i]][path[i + 1]];
    }
    cost += graph[path[path.len()-1]][0];
    cost
}

fn randompath(graph: &Vec<Vec<i32>>, rng: &mut ThreadRng) -> (i32, Vec<usize>) {

    let mut ver :Vec<usize> = (1..graph.len()).collect();
    ver.shuffle(rng);

    (calculate_cost(graph, &ver), ver)
}
//Choosing a path to start calculating with. Best out of x samples in a graph
fn choosefirst(graph: &Vec<Vec<i32>>, sample: usize) -> (i32, Vec<usize>) {
    let mut rng = thread_rng();

    let mut best_solution = (i32::MAX, vec![]);

    for _ in 0..sample{
        let solution = randompath(graph, &mut rng);
        if solution.0 < best_solution.0
        {
            best_solution = solution;
        }

    }


    best_solution
}

pub fn swap(graph: &Vec<Vec<i32>>, path: &Vec<usize>,j:usize ,k:usize) -> (i32, Vec<usize>){
    let mut afterpath = path.clone();
    afterpath.swap(j,k);

    (calculate_cost(graph,&afterpath),afterpath)
}

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

pub fn cadlen() -> usize{
    //return (sollen/(sollen-tabulen)) * 10 - 5 + ((64 - n.leading_zeros()) * 2) as usize
    return 40
}


pub fn tabusearch_v2(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){
    let n = graph.len();
    let mut solution = choosefirst(graph, 1000);
    let mut optsolution = solution.clone();
    let tabulen = 25;
    let mut increment = 1;
    let aspiration = solution.0/n as i32;
    let mut tabulist: Vec<(i32,i32,i32)> = vec![];
    let mut maxincrement = 200;
    loop {
        let mut solutions = PriorityQueue::new();
        for i in 0..graph.len()-1 {
            for j in i+1..graph.len()-1 {
                if !tabulist.iter().any(|&(x, y, z)| x == i as i32 && y == j as i32) {
                    let potential = invert(graph, &solution.1,i,j);
                    solutions.push((potential.1,i,j), -potential.0);
                }

            }
        }
        let mut tmpsolution = solutions.peek().expect("Should not be empty");

        increment += 1;
        if -tmpsolution.1 < optsolution.0{
            optsolution = (-tmpsolution.1, tmpsolution.0.0.clone()); //if tabulist.len() < tabulen {
            println!("New best: {}, iteration: {}", optsolution.0, increment);
          maxincrement = increment + 1000;
        }
        else {
                if -tmpsolution.1 >= solution.0 {
                    tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen() as i32));
                }
        }

        let mut asp = false;
        let mut baspsol = (1,vec![0 as usize]);
        //antyperspiracja
        for item in &tabulist{
            let aspsol = invert(graph, &solution.1, item.0 as usize, item.1 as usize);
            if aspsol.0 < optsolution.0 || (aspsol.0 < solution.0-aspiration && aspsol.0 < -tmpsolution.1-aspiration)
            {
                if aspsol.0 < optsolution.0 {
                    optsolution = aspsol.clone();
                }
                println!("Aspiration activated:");
                println!("Discarded solution: {}, Previous solution: {}, Aspsolution {}", -tmpsolution.1, solution.0, aspsol.0);
                asp = true;
                baspsol = aspsol;
            }
        }

        if asp {
            solution = baspsol;
        } else {
            solution = (-tmpsolution.1, tmpsolution.0.0.clone());
        }

        if increment > maxincrement {
            break;
        }

        let mut indices_to_remove: Vec<usize> = Vec::new();
        // Decrease the third i32 in every element
        for (index, tuple) in tabulist.iter_mut().enumerate() {
            tuple.2 -= 1; // Decrease the third element by 1

            // Check if the third element is zero and mark the index for removal
            if tuple.2 == 0 {
                indices_to_remove.push(index);
            }
        }

        // Remove tuples with third element equal to 0
        for &index in indices_to_remove.iter().rev() {
            tabulist.remove(index);
        }

        while tabulist.len() > tabulen {
            tabulist.remove(0);
        }
    }
    optsolution
}