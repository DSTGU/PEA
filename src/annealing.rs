use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

fn calculate_cost(graph: &Vec<Vec<i32>>, path: &Vec<usize>) -> i32
{
    let mut cost = graph[0][path[0]];
    for i in 0..path.len() - 1 {
        cost += graph[path[i]][path[i + 1]];
    }
    cost += graph[path[path.len()-1]][0];
    cost
}
fn randompath(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {
    let mut rng = thread_rng();
    let mut ver :Vec<usize> = (1..graph.len()).collect();
    ver.shuffle(&mut rng);

    (calculate_cost(graph, &ver), ver)
}

fn swap (graph: &Vec<Vec<i32>>, path: &Vec<usize>, j:usize , k:usize) -> (i32, Vec<usize>){
    let mut afterpath = path.clone();
    afterpath.swap(j,k);

    (calculate_cost(graph,&afterpath),afterpath)
}

fn starttemperature(graph: &Vec<Vec<i32>>, sample: i32) -> f32 {

    let mut difference : f32  = 0.0;
    let mut rng = thread_rng();
    let n = graph.len() - 1;
    for _ in 0..sample {
        let path = randompath(graph);
        difference += (path.0 - swap(graph, &path.1, rng.gen_range(0..n), rng.gen_range(0..n)).0).abs() as f32;

    }

    difference/sample as f32

}


fn probability(delta:f32, temperature: f32) -> f32 {
    f32::powf(std::f32::consts::E, -delta/temperature)
}

fn nextTLundy(temperature: f32, lambda: f32) -> f32 {
    temperature/(1.0+0.0005*temperature)
}

pub fn annealing(graph: &Vec<Vec<i32>>, lambda: f32) -> (i32, Vec<usize>)
{
    let mut solution = randompath(graph);

    let mut rng = thread_rng();
    let n = graph.len()-1;
    let mut temperature = starttemperature(graph, 10000);

    loop {

        for _ in 0..(2* n.pow(2)) {

            let swapsolution = swap(graph, &solution.1, rng.gen_range(0..n), rng.gen_range(0..n));

            if rng.gen::<f32>() < probability((swapsolution.0 - solution.0 )as f32, temperature){
                solution = swapsolution;
            }



        }
        //temperature = temperature * lambda;
        temperature = nextTLundy(temperature, lambda);
        if temperature < 1.005 {
            println!("{}", temperature);
            break;
        }



    }

    solution
}