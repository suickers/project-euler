fn next_permutation(v: &mut [u32]) -> bool {
    let n = v.len();
    if n < 2 { return false; }
    let mut i = n - 2;
    while i > 0 && v[i] >= v[i + 1] {
        i = i.saturating_sub(1); 
    } 
    if i == 0 { return false; }
    
    
    let mut j = n - 1; 
    while j > i && v[j] <= v[i] {
        j = j.saturating_sub(1);
    }
    
    v.swap(i, j);
    
    v[i+1..].reverse();
    
    true
}

fn find_all_permutation(mut v: Vec<u32>) -> Vec<Vec<u32>> {
    let mut vec = Vec::new();
    
    vec.push(v.clone());
    
    while next_permutation(&mut v) {
        vec.push(v.clone());
    }
    
    vec
}

fn main() {
    let v = vec![0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    let mut vec = find_all_permutation(v).into_iter().nth(999_999).unwrap();
    let _r = vec.remove(0);
    let result = vec.iter().fold(0, |a, &n| a * 10 + n as u64);
    assert_eq!(result, 2783915460);
}
