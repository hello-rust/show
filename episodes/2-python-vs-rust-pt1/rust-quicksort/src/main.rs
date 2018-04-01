fn quicksort(items: Vec<u64>) -> Vec<u64> {
    let pivot = match items.get(0) {
        Some(i) => i,
        _ => return vec![],
    };
    let less = items.iter().map(|&i| i).filter(|i| i < pivot).collect();
    let more = items
        .iter()
        .skip(1)
        .map(|&i| i)
        .filter(|i| i >= pivot)
        .collect();
    [quicksort(less), vec![*pivot], quicksort(more)].concat()
}

fn main() {
    println!("{:?}", quicksort(vec![6, 2, 7, 2, 3, 4]));
}
