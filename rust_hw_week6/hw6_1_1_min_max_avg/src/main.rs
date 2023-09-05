fn min_max_avg(v: &Vec<f32>) -> (f32, f32, f32){
    let mut max = 0.;
    let mut min = v[0];
    for i in v{
        if i > &max{
            max = *i
        }
        if i < &min{
            min = *i
        }
    }
    let sum: f32 = v.iter().sum();
    let avg = sum/v.len() as f32;
    return (min,max,avg)
}

fn main() {
    let test_vec = vec![1.,2.,3.,8.,5.,6.,7.];
    print!("{:?}", min_max_avg(&test_vec))
}

#[test]
fn test_minmaxave() {
    assert_eq!(min_max_avg(&vec![1.,2.,3.,4.,5.]), (1.,5.,3.));
    assert_eq!(min_max_avg(&vec![-5.,-3.,-1.,1.,3.,5.]), (-5.,5.,0.));
    assert_eq!(min_max_avg(&vec![-144.,21.545,3111.2,-43.2,5.9764316]), (-144.,3111.2,590.30428632));
}