use rand::Rng;

pub fn parse_graph_matrix(input: &str) -> Vec<Vec<i32>>{
    let lines: Vec<&str> = input.lines().collect();
    //print!("{:?}", lines);
    let t = lines[0].trim();
    let n: usize = lines[1].trim().parse().expect("Failed to parse graph size");
    let mut graph_matrix = vec![vec![0; n]; n]; // Initialize an NxN matrix with zeros


    if t == "Explicit" {
        let mut allv = Vec::with_capacity(n*n);

        for i in 2..lines.len() {
            let line = lines[i];
            let values: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().expect("Failed to parse matrix value"))
                .collect();

            allv.extend_from_slice(&values);
        }
        //println!("{} {}", graph_matrix.len(), allv.len());
        for i in 0..n{
            graph_matrix[i] = allv[i*n..(i+1)*n].to_vec()
        }

        //println!("graph matrix: {:?}", graph_matrix);

    }

    if t == "Coord" {
        let mut x : Vec<f32> = vec![];
        let mut y : Vec<f32> = vec![];

        for i in 2..lines.len() {
            let mut a = lines[i].split_whitespace();
            a.next();
            x.push(a.next().expect("Iterator stuff failed").trim().parse().expect("Int assertion failed"));
            y.push(a.next().expect("Iterator stuff failed").trim().parse().expect("Int assertion failed"));
        }
        //println!("{:?}", x);
        //println!("{:?}", y);

        for f in 0..n{
            for s in 0..n {
                graph_matrix[f][s] = ((x[f] - x[s]).powf(2 as f32) + (y[f] - y[s]).powf(2 as f32)).sqrt() as i32;
            }
        }
        //println!("{:?}", graph_matrix);
    }



    graph_matrix
}

pub(crate) fn generate_graph(n: i32, max: i32) -> (Vec<Vec<i32>>)
{
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..n   {
        let mut subv = vec![];
        for _ in 0..n
        {
            subv.push(rng.gen_range(0..max));
        }
        v.push(subv);
    }

    //dbg!(v.clone());
    return v;
}