fn cal_partial_sums(v: &Vec<f32>) -> Vec<f32>{
    let mut return_vector = Vec::new();
    let mut current_sum = 0.;
    for i in v{
        current_sum += i;
        return_vector.push(current_sum);
    }
    return_vector
}

fn main() {
    // let test_vec = vec![2., 11., 3., 4., 7.];
    let test_vec = vec![1.,-16.,3.,-956.,546.];
    println!("{:?}", cal_partial_sums(&test_vec))
}

#[test]
fn test_calpartialsums(){
    assert_eq!(cal_partial_sums(&vec![1.,2.,3.,4.,5.]),vec![1.,3.,6.,10.,15.]);
    assert_eq!(cal_partial_sums(&vec![1.,-2.,3.,-4.,5.]),vec![1.,-1.,2.,-2.,3.]);
    assert_eq!(cal_partial_sums(&vec![1.,-16.,3.,-956.,546.]),vec![1., -15., -12., -968., -422.])
}