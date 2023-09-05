use std::cmp;

fn pack_number_tuples_s3(v1: &Vec<i32>, v2: &Vec<i32>, v3: &Vec<i32>) -> Vec<(i32, i32, i32)>{
    let mut return_vector = Vec::new();
    let shortest_vec = cmp::min(cmp::min(v1.len(), v2.len()), v3.len());
    for i in 0..shortest_vec{
        return_vector.push((v1[i], v2[i], v3[i]))
    }
    return_vector
}

fn main() {
    print!("{:?}", pack_number_tuples_s3(&vec![1, 2], &vec![4, 3], &vec![5]))
}

#[test]
fn test_packnumbertupless3(){
    assert_eq!(pack_number_tuples_s3(&vec![1, 2], &vec![4, 3], &vec![5]),vec![(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&vec![1, 1, , 1, 1, 1, 1, 1], &vec![1], &vec![1]),vec![(1, 1, 1)]);
    assert_eq!(pack_number_tuples_s3(&vec![-1, -2], &vec![4, 3], &vec![-5]),vec![(-1, 4, -5)]);
    assert_eq!(pack_number_tuples_s3(&vec![1,2,3], &vec![4,5], &vec![6]),vec![(1, 4, 6)])
}