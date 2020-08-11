use rand::prelude::*;

fn main() {
    let mut r = rand::thread_rng();
    let mut h = 0;
    let mut b = 0;
    let m =100000;
    let f = (m as f64);
    for _ in 0..m
    {
        h += t(0.32,6,&mut r) as i32;
        b += t(0.40,5,&mut r) as i32;
    }
    println!("Hős: {}/{} = {}",h,m,(h as f64)/f);
    println!("Bela: {}/{} = {}",b,m,(b as f64)/f);

}
fn t(x:f64,n:u8,r:&mut ThreadRng)->bool
{
    let mut c= 0;
    for _ in 0..=n
    {
        if r.gen::<f64>() <= x {c+=1}
    }
    c <= 2
}
