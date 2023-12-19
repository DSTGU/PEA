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

fn similarity(ogpath: &Vec<usize>, path: &Vec<usize>) -> usize {
    let mut sim = 0;

    for i in 0..ogpath.len(){
        if ogpath[i] == path[i]
        {
            sim += 1;
        }
    }

    sim
}

fn randompath(graph: &Vec<Vec<i32>>, rng: &mut ThreadRng) -> (i32, Vec<usize>) {

    let mut ver :Vec<usize> = (1..graph.len()).collect();
    ver.shuffle(rng);

    (calculate_cost(graph, &ver), ver)
}

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

pub fn localsearch(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){

    let mut solution = choosefirst(graph, 1000);
    //println!("LS Iteration 0, path: {:?}, cost: {}", solution.1, solution.0);
    let mut increment = 0;

    loop {
        for i in 0..graph.len()-1 {
            for j in 0..graph.len()-1 {
                let potential = swap(graph, &solution.1,i,j);
                if potential.0 < solution.0
                {
                    increment += 1;
                    solution = potential;
                    //println!("LS Iteration {}, path: {:?}, cost: {}", increment, solution.1, solution.0);
                    continue;
                }
            }
        }


        break;
    }





    solution
}

pub fn localsearch_best(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){

    let mut solution = choosefirst(graph, 1000);
    //println!("LS Iteration 0, path: {:?}, cost: {}", solution.1, solution.0);
    let mut increment = 0;

    loop {
        let mut solutions = vec![];
        for i in 0..graph.len()-1 {
            for j in 0..graph.len()-1 {
                let potential = swap(graph, &solution.1,i,j);
                if potential.0 < solution.0
                {
                    solutions.push(potential);
                }
            }
        }

        //println!("{:?}", solutions);

        for sol in solutions {
            if sol.0 < solution.0 {
                solution = sol;
            }
        }

        increment += 1;
        //println!("LS Iteration {}, path: {:?}, cost: {}", increment, solution.1, solution.0);
        break;
    }

    solution
}

pub fn localsearch_best_PQ(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){
    let mut solution = choosefirst(graph, 1000);
    //println!("LS Iteration 0, path: {:?}, cost: {}", solution.1, solution.0);
    let mut minval = solution.0;
    let mut increment = 0;

    loop {
        let mut solutions = PriorityQueue::new();
        for i in 0..graph.len()-1 {
            for j in 0..graph.len()-1 {
                let potential = swap(graph, &solution.1,i,j);
                    solutions.push(potential.1, -potential.0);
            }
        }



        //println!("{:?}", solutions.peek());

        solution = (-solutions.peek().expect("Should not be empty").1, solutions.peek().expect("Should not be empty").0.to_vec());

        increment += 1;
        //println!("LS Iteration {}, path: {:?}, cost: {}", increment, solution.1, solution.0);
        if solution.0 >= minval{
            break;
        }
        minval = solution.0
    }
    solution
}

pub fn cadlen(sollen: usize, tabulen: usize, n: usize) -> usize{
    //return (sollen/(sollen-tabulen)) * 10 - 5 + ((64 - n.leading_zeros()) * 2) as usize
    return 40
}

pub fn tabusearch_v1(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>){
    let n = graph.len();
    let mut solution = choosefirst(graph, 1000);
    let mut optsolution = solution.clone();
    let tabulen = 15;
    //println!("LS Iteration 0, path: {:?}, cost: {}", solution.1, solution.0);
    let mut increment = 0;
    let mut tabulist: Vec<(i32,i32,i32)> = vec![];
    let mut maxincrement = 500000/n;
    loop {
        let mut solutions = PriorityQueue::new();
        for i in 0..graph.len()-1 {
            for j in i+1..graph.len()-1 {
                if !tabulist.iter().any(|&(x, y, z)| x == i as i32 && y == j as i32) {
                    let potential = swap(graph, &solution.1,i,j);
                    solutions.push((potential.1,i,j), -potential.0);
                }

            }
        }

        //println!("{:?}", solutions.peek());

        let tmpsolution = solutions.peek().expect("Should not be empty");

        increment += 1;
        //println!("LS Iteration {}, path: {:?}, cost: {}", increment, solution.1, solution.0);
        if -tmpsolution.1 < optsolution.0{
            optsolution = (-tmpsolution.1, tmpsolution.0.0.clone());
            //if tabulist.len() < tabulen {
            //    tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen(solutions.len(), tabulist.len(), n + 15) as i32));
            //}
            println!("New best: {}, iteration: {}", optsolution.0, increment);
            maxincrement = increment + 500000/n;
        }
        else {
            if tabulist.len() < tabulen {
                if -tmpsolution.1 > solution.0 {
                    tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen(solutions.len(), tabulist.len(), n + 5) as i32));
                }
            }
        }

        solution = (-tmpsolution.1, tmpsolution.0.0.clone());

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

        //println!("Solution: {:?}",solution);
        //println!("Tabulist: {:?}",tabulist );

    }
    optsolution
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
                    tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen(solutions.len(), tabulist.len(), n + 5) as i32));
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

/*pub enum Aspiration{
    None,
    Best,
    Improve(i32),
    Combined(i32),
}

pub enum Endcriteria{
    Time(usize),
    GlobalIteration(usize, usize),
    ImproveIteration(usize, usize, usize),
}

pub enum CriticalEvents{
    Similarity,
    Looping(usize),
    No,
}

// Use default : ### Cadence - 1, Tabulen: 15, Aspiration: None, Endcriteria: ImproveIteration (1000,1000) - Depth = when reaching criticalevent how much further to look
pub fn tabusearch_test(graph: &Vec<Vec<i32>>, cadence: usize, tabulen: usize, aspiration: Aspiration, endcriteria: Endcriteria, critical : CriticalEvents) -> (i32, Vec<usize>){
    let n = graph.len();
    let mut solution = choosefirst(graph, 1000);
    let mut originalsolution = solution.clone();
    let mut optsolution = solution.clone();
    //println!("LS Iteration 0, path: {:?}, cost: {}", solution.1, solution.0);
    let mut increment: usize = 1;
    let mut tabulist: Vec<(i32,i32,i32)> = vec![];
    let mut maxincrement = 1000;
    loop {
        let mut solutions = PriorityQueue::new();
        for i in 0..graph.len()-1 {
            for j in i+1..graph.len()-1 {
                if !tabulist.iter().any(|&(x, y, z)| x == i as i32 && y == j as i32) {
                    let potential = swap(graph, &solution.1,i,j);
                    solutions.push((potential.1,i,j), -potential.0);
                }

            }
        }

        //println!("{:?}", solutions.peek());

        let mut tmpsolution = solutions.peek().expect("Should not be empty");

        increment += 1;
        //println!("LS Iteration {}, path: {:?}, cost: {}", increment, solution.1, solution.0);
        if -tmpsolution.1 < optsolution.0{
            optsolution = (-tmpsolution.1, tmpsolution.0.0.clone());
            if tabulist.len() < tabulen {
                tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen(solutions.len(), tabulist.len(), n + 10*cadence) as i32));
            }
            println!("New best: {}, iteration: {}", optsolution.0, increment);
            maxincrement = increment + 1000;
        }
        else {
            if tabulist.len() < tabulen {
                tabulist.push((tmpsolution.0.1 as i32, tmpsolution.0.2 as i32, cadlen(solutions.len(), tabulist.len(), n + 5 * cadence) as i32));
            }
        }


        let mut asp = false;
        let mut baspsol = (1,vec![0 as usize]);

        //antyperspiracja
        match aspiration {
            Aspiration::None => (),
            Aspiration::Best =>
                for item in &tabulist{
                    let aspsol = swap(graph, &solution.1, item.0 as usize, item.1 as usize);
                    if aspsol.0 < optsolution.0{
                        if aspsol.0 < optsolution.0 {
                            optsolution = aspsol.clone();
                        }
                        println!("Aspiration activated:");
                        println!("Discarded solution: {}, Previous solution: {}, Aspsolution {}", -tmpsolution.1, solution.0, aspsol.0);
                        asp = true;
                        baspsol = aspsol;
                    }
                },
            Aspiration::Improve(aspiration) =>
                for item in &tabulist{
                    let aspsol = swap(graph, &solution.1, item.0 as usize, item.1 as usize);
                    if aspsol.0 < solution.0-aspiration && aspsol.0 < -tmpsolution.1-aspiration {
                        if aspsol.0 < optsolution.0 {
                            optsolution = aspsol.clone();
                        }
                        println!("Aspiration activated:");
                        println!("Discarded solution: {}, Previous solution: {}, Aspsolution {}", -tmpsolution.1, solution.0, aspsol.0);
                        asp = true;
                        baspsol = aspsol;
                    }
                }
            Aspiration::Combined(aspiration) =>
                for item in &tabulist{
                    let aspsol = swap(graph, &solution.1, item.0 as usize, item.1 as usize);
                    if aspsol.0 < solution.0-aspiration && aspsol.0 < -tmpsolution.1-aspiration || aspsol.0 < optsolution.0{
                        if aspsol.0 < optsolution.0 {
                            optsolution = aspsol.clone();
                        }
                        println!("Aspiration activated:");
                        println!("Discarded solution: {}, Previous solution: {}, Aspsolution {}", -tmpsolution.1, solution.0, aspsol.0);
                        asp = true;
                        baspsol = aspsol;
                    }
                }
        }
        if asp {
            solution = baspsol;
        } else {
            solution = (-tmpsolution.1, tmpsolution.0.0.clone());
        }



        //CriticalEvents
        match critical {
            CriticalEvents::No => (),
            CriticalEvents::Similarity=> if similarity( &originalsolution.1, &solution.1) > n / (64 - increment.leading_zeros()) {
                let innersolution = match endcriteria {
                    Endcriteria::ImproveIteration(maxiter, curriter, bestsol) => tabusearch_test(graph, cadence, tabulen, aspiration, Endcriteria::ImproveIteration(maxiter, curriter, optsolution.0 as usize) // TODO: FIX, critical),
                    _ => tabusearch_test(graph, cadence, tabulen, aspiration, endcriteria, critical),
                }

                if innersolution.1 < optsolution.1{
                    return innersolution
                } else
                {
                    return optsolution
                }

            } ,
            CriticalEvents::Looping(loophash) =>  (),
        }





        match endcriteria {
            Endcriteria::Time(sec) => (),
            Endcriteria::GlobalIteration(Curriter, Iter) => if increment > Iter { return optsolution },
            Endcriteria::ImproveIteration(Curriter, Maxiter, bestsolution) => if increment > maxincrement { return optsolution},
        }



        // Tabulist management
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

        //println!("Solution: {:?}",solution);
        //println!("Tabulist: {:?}",tabulist );

    }
    optsolution
}*/