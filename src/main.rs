/************* Cargos *************/

use std::{env, time::{Instant, Duration}, ops::Add};
use rand::Rng;
use std::iter;
use rayon::{prelude::{IntoParallelIterator, ParallelIterator}, str::ParallelString};

/************* Auxiliary Functions *************/

fn add_bits(b1: char, b2: char, c: char) -> (char, char){
    let s: u32 = [b1,b2,c].into_iter().map(|e| e.to_digit(10).unwrap()).sum();
    (char::from_digit(s%2, 10).unwrap(),char::from_digit(s/2, 10).unwrap())
}

fn pad_binaries(b1: String, b2: String) -> (String, String) {
    // Initial 0 for the eventual last carry
    let mut o1: String = "0".to_string();
    let mut o2: String = "0".to_string();

    let (l1, l2) = (b1.len(), b2.len());

    // Adding the padding 0s to have the 2 String be the same size
    if l1 == l2 {}
    else if l1 > l2 {o2.push_str(&std::iter::repeat('0').take(l1-l2).collect::<String>());}
    else if l1 < l2 {o1.push_str(&std::iter::repeat('0').take(l2-l1).collect::<String>());}

    // Adding the actual numbers
    o1.push_str(&b1);o2.push_str(&b2);

    (o1,o2)
}

fn propagate_carry(s:String, c: char) -> (String,char){
    if c=='0'{
        return (s.to_string(),'0');
    }
    let last_0 = s.chars().rev().position(|e|e=='0');
    if last_0 == None {
        return (s.par_chars().into_par_iter().map(|_| '0').collect(), '1')
    }
    let begin = s.len() - last_0.unwrap()-1;
    let mut s_bis = s.clone().to_string();
    s_bis.replace_range(
        begin..s.len(),
        &"1".chars().chain(iter::repeat('0').take(last_0.unwrap())).collect::<String>()
    );
    (s_bis.clone(),'0')
}

/************* Add Binary *************/

#[allow(dead_code)]
pub fn seq_add_binary_v1(a: String, b: String) -> String {
    // Padding the strings with 0s to be the same length with an extra 0
    let (b1,b2) = pad_binaries(a, b);
    // This will carry our result
    let mut result: String = "".to_string();

    let iter = b1.chars().rev().into_iter().zip(b2.chars().rev().into_iter());
    let mut c: char = '0';
    let mut b: char;
    for e in iter {
        (b,c) = add_bits(e.0, e.1, c);
        result = b.to_string() + &result;
    }
    //c.to_string() + &result
    result
}

#[allow(dead_code)]
pub fn seq_add_binary_v2(a: String, b: String) -> String {
    // Padding the strings with 0s to be the same length with an extra 0
    let (b1,b2) = pad_binaries(a, b);

    b1.chars().rev().into_iter()
    .zip(b2.chars().rev().into_iter())
    .fold(("".to_string(),'0'), |(res,c),(a,b)| {
        let out = add_bits(a,b,c);
        (out.0.to_string() + &res,out.1)
    }).0
}

fn par_add_rec(a: &str, b: &str, level: u32) -> (String, char){
    if level==1 {
    return a.to_string().chars().rev().into_iter()
        .zip(b.to_string().chars().rev().into_iter())
        .fold(("".to_string(),'0'), |(res,c),(a,b)| {
            let out = add_bits(a,b,c);
            (out.0.to_string() + &res,out.1)
        })
    }
    assert_eq!(a.len(),b.len());
    let len = a.len();
    let mid = len/2;

    let (l_a, r_a, l_b, r_b) = (
        &a[0..mid],
        &a[mid..len],
        &b[0..mid],
        &b[mid..len]
    );

    let ((l_o,l_c),(r_o,r_c)) = diam::join(
        || par_add_rec(l_a,l_b,level-1), 
        || par_add_rec(r_a,r_b,level-1)
    );

    let mut output: String = l_o.clone();
    let mut output_carry: char = l_c;

    if r_c == '1' {
        let test: char;
        (output, test) = propagate_carry(l_o, r_c);
        if l_c=='1' {assert_eq!(test,'0');}
        else {output_carry = test;}
    }
    output.push_str(&r_o);
    (output, output_carry)
}

pub fn par_add_binary(a: String, b: String, level: u32) -> String{
    assert!(level>0);
    let (b1,b2) = pad_binaries(a, b);
    let (s_o,_) = par_add_rec(&b1, &b2, level);
    s_o
}

/************* Main *************/
fn main() {
    let mut size_lower_bound: u32 = 1<<18;

    let iterations: u32 = 100;

    let mut seq_times: Duration;
    let mut par_times_1: Duration;
    let mut par_times_2: Duration;
    let mut par_times_3: Duration;
    let mut par_times_4: Duration;
    let mut par_times_5: Duration;

    
    let mut rng = rand::thread_rng();


    for size in 10..21{
        size_lower_bound = 1<<size;
        seq_times = Duration::new(0, 0);
        par_times_1 = Duration::new(0, 0);
        par_times_2 = Duration::new(0, 0);
        par_times_3 = Duration::new(0, 0);
        par_times_4 = Duration::new(0, 0);
        par_times_5 = Duration::new(0, 0);

        for _ in 0..iterations {
            let a: String = (0..size_lower_bound).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
            let b: String = (0..size_lower_bound).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
            
            // Sequential Version
            let start = std::time::Instant::now();
            let seq_res = seq_add_binary_v2(a.clone(), b.clone());
            seq_times += start.elapsed();

            // Parallel Version
            let start = std::time::Instant::now();
            let par_res = par_add_binary(a.clone(), b.clone(),1);
            par_times_1 += start.elapsed();
            assert_eq!(seq_res,par_res);

            let start = std::time::Instant::now();
            let par_res = par_add_binary(a.clone(), b.clone(),2);
            par_times_2 += start.elapsed();
            assert_eq!(seq_res,par_res);

            let start = std::time::Instant::now();
            let par_res = par_add_binary(a.clone(), b.clone(),3);
            par_times_3 += start.elapsed();
            assert_eq!(seq_res,par_res);

            let start = std::time::Instant::now();
            let par_res = par_add_binary(a.clone(), b.clone(),4);
            par_times_4 += start.elapsed();
            assert_eq!(seq_res,par_res);

            let start = std::time::Instant::now();
            let par_res = par_add_binary(a.clone(), b.clone(),5);
            par_times_5 += start.elapsed();    
            assert_eq!(seq_res,par_res);
        }
        println!("###########################################");
        println!("Settings : Size={}",size_lower_bound);
        println!("Average Sequential Time : {:?}",seq_times/iterations);
        println!("Average Parallel Time (levels={}) : {:?}",1,par_times_1/iterations);
        println!("Average Parallel Time (levels={}) : {:?}",2,par_times_2/iterations);
        println!("Average Parallel Time (levels={}) : {:?}",3,par_times_3/iterations);
        println!("Average Parallel Time (levels={}) : {:?}",4,par_times_4/iterations);
        println!("Average Parallel Time (levels={}) : {:?}",5,par_times_5/iterations);
        println!("###########################################");
    }
}
