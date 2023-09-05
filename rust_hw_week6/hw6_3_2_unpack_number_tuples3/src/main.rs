fn unpack_number_tuples(v: &Vec<(i32, i32, i32)>) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut return_vec1 = Vec::new();
    let mut return_vec2 = Vec::new();
    let mut return_vec3 = Vec::new();
    for i in v{
        return_vec1.push(i.0)
    }
    for i in v{
        return_vec2.push(i.1)
    }
    for i in v{
        return_vec3.push(i.2)
    }
    (return_vec1, return_vec2, return_vec3)
}

fn main() {
    print!("{:?}", unpack_number_tuples(&vec![(1, 4, 5), (2, 2, 1)]));
}

#[test]
fn test_packnumbertupless3(){
    assert_eq!(unpack_number_tuples(&vec![(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
    assert_eq!(unpack_number_tuples(&vec![(1, 4, 5), (2, 2, 1), (5,3,7)]), (vec![1, 2, 5], vec![4, 2, 3], vec![5, 1, 7]));
    assert_eq!(unpack_number_tuples(&vec![(1, -4, 5), (2, -2, 1), (-5,3,7)]), (vec![1, 2, -5], vec![-4, -2, 3], vec![5, 1, 7]));
}