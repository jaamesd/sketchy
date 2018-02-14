mod tdigest;

#[derive(Debug)]
enum Enum {
    Foo0 = 0,
    Foo1 = 1,
}


fn main() {
    let c0 = tdigest::Centroid::new(10.0, 11.0);
    let c1 = tdigest::Centroid::new(1.0, 2.0);
    let mut v0 = vec!{c0, c1};
    *v0.last_mut().unwrap() += c0;

    println!("Centroid {:?}", v0.last());

    let mut digest = tdigest::Tdigest::new(1000.0, 10000);

    let max = 1000000 * 2;
    for x in 0..max
    {
        // println!("x {:?}", x);
        digest.merge_sample(f64::from(x));
        // digest.merge_sample(f64::from(x) + 0.5);
        // digest.merge_sample(f64::from(max - x));
    }

    digest.merge_centroids();

    // for x in 0..101
    // {
    //     let q = f64::from(x) /100.0;
    //     let delta = 6.0;
    //     let k = tdigest::Tdigest::scaling_function(q, delta);
    //     let q2 = tdigest::Tdigest::inv_scaling_function(k, delta);
    //     println!("q {:.5} k {:.5} q' {:.5}", q, k ,q2 );
    // }

    println!("digest {:?}", digest);
}
