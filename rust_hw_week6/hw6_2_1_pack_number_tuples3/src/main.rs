use std::cmp;

fn pack_number_tuples3(v1: &Vec<i32>, v2: &Vec<i32>, v3: &Vec<i32>) -> Vec<(i32, i32, i32)>{
    let mut return_vector = Vec::new();
    let (mut vec1, mut vec2, mut vec3) = (Vec::new(), Vec::new(), Vec::new());
    let longest_vec = cmp::max(cmp::max(v1.len(), v2.len()), v3.len());
    for i in v1{
        vec1.push(*i)
    }
    for i in v2{
        vec2.push(*i)
    }
    for i in v3{
        vec3.push(*i)
    }
    for _ in 0..longest_vec - vec1.len(){
        vec1.push(0)
    }
    for _ in 0..longest_vec - vec2.len(){
        vec2.push(0)
    }
    for _ in 0..longest_vec - vec3.len(){
        vec3.push(0)
    }
    for i in 0..longest_vec{
        return_vector.push((vec1[i], vec2[i], vec3[i]))
    }
    return_vector
}

fn main() {
    print!("{:?}", pack_number_tuples3(&vec![1, 2], &vec![4, 3], &vec![5]))
}

#[test]
fn test_packnumbertupless3(){
    assert_eq!(pack_number_tuples3(&vec![1, 2], &vec![4, 3], &vec![5]),vec![(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&vec![1, 1, 1, 1, 1, 1, 1, 1], &vec![1], &vec![1]),vec![(1, 1, 1), (1, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0), (1, 0, 0)]);
    assert_eq!(pack_number_tuples3(&vec![-1, -2], &vec![4, 3], &vec![-5]),vec![(-1, 4, -5), (-2, 3, 0)])
}