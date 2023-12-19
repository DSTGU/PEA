use rand::Rng;

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
    let mut mapping = vec![0; len];
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