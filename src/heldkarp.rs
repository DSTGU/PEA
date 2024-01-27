use std::collections::HashMap;
use itertools::FoldWhile::Continue;
use itertools::min;


pub fn held_karp(graph: &Vec<Vec<i32>>) -> (i32, Vec<usize>) {

    let size = graph.len();
    // mask, node -> value
    let mut vec_hash = vec![];
    let mut map1 = HashMap::new();

    for i in 1..size {
        map1.insert((1 << i, i), graph[0][i]);
    }

    vec_hash.push(map1);

    for _ in 1..size-1 {
        //println!("ITERATION: {}", it);
        let mut mapit = HashMap::new();
        for (k,v) in vec_hash.last().expect("") {
            for i in 1..size{
                if k.0 & 1 << i != 0 {
                    continue;
                }
                mapit.entry((k.0 | 1 << i ,i)).and_modify(|existing_value| {
                    if v+graph[k.1][i] < *existing_value {
                        *existing_value = v + graph[k.1][i];
                    }}).or_insert(v + graph[k.1][i]);
            }
        }
        vec_hash.push(mapit);
    }

    // node, num -> mask, value tuples
    let mut minval = i32::MAX;
    let mut minprev = 0;
    for (k, v) in vec_hash.last().expect("plswork") {
        //println!("MIN: {}, NODE: {}, VAL:{}", minval, k.1 ,v+graph[k.1][0]);

        if v+graph[k.1][0] < minval
        {
            minval = v + graph[k.1][0];
            minprev = k.1;
        }
    }

    //We ve got the buck, now recover the path
    let mut path = vec![];
    let mut curminval = minval;
    let mut curmin = 0;
    let mut curmask = 2_usize.pow(size as u32)-1;


    for i in (0..vec_hash.len()).rev(){
        for (k, v) in &vec_hash[i] {
            //println!("NODE: {}, VAL:{}, MASK: {:#06b}", k.1 ,v+graph[k.1][0], k.0);

            //println!("v + graph[k.1][curmin]: {}", v + graph[k.1][curmin]);
            //println!("K0: {:#016b}, curmask: {:#016b}, comb: {:#016b}, fun: {}", k.0, curmask, (k.0 & curmask), (k.0 & curmask).count_ones());
            if v + graph[k.1][curmin] == curminval && (k.0 ^ curmask).count_ones() == 1 /*&& curmask & 1 << k.1 != 0*/  && k.1 != curmin
            {
                minprev = k.1;
                //println!("Entry, Edge under investigation: {} - {}, mask: {:#019b}", k.1, curmin, k.0);
            }
        }
        curmask = curmask ^ 1 << curmin;
        curminval -= graph[minprev][curmin];
        //println!("iter: {}, val: {}, Edge: {} - {} - cost: {}, Mask: {:#019b}", i, curminval, minprev, curmin, graph[minprev][curmin], curmask);

        path.push(minprev);
        curmin = minprev;
    }

    //println!("EDGE: 0 - {}, Cost: {}", curmin, graph[0][curmin]);

    //path.push(minprev);
    //path.push(0);
      path.reverse();
    //path.pop();

    (minval, path)
}