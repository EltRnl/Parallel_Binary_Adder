/************* Cargos *************/

use std::env;
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
    let help: String = 
    "Usage : cargo run -- [Size] [Levels] [Iterations]
        > Size : used to calculate the size lower bound 2^Size 
        (default Size = 18)
        > Levels : the number of levels in the parallel recusion, 
        creating 2^(Levels-1) parallel process (default Levels = 4)
        > Iterations : number of time we executes the calculation 
        with random numbers each time (default 1)
    ".to_string();
    println!("{}",help);

    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    assert!(argc<5);

    let mut size_lower_bound: u32 = 1<<18;
    if argc>1 {
        match args[1].parse::<u32>() {
            Ok(v) =>  {size_lower_bound = 1<<v; println!("Size given {}.",v);},
            Err(_e) => {println!("Could not read Size '{}', using default Size=18.",args[1]);},
        }
    }else{
        println!("No value given for Size, using default Size=18.");
    }
    
    let mut levels: u32 = 4;
    if argc>2 {
        match args[2].parse::<u32>() {
            Ok(v) =>  {levels = v; println!("Levels given {}.",v);}, 
            Err(_e) => {println!("Could not read Levels '{}', using default Levels=4.",args[2]);},
        }
    }else{
        println!("No value given for Levels, using default Levels=4.");
    }

    let mut iterations: u32 = 1;
    if argc>3 {
        match args[3].parse::<u32>() {
            Ok(v) =>  {iterations = v; println!("Iteration given {}.",v);},
            Err(_e) => {println!("Could not read Iterations '{}', using default Iterations=1.",args[3]);},
        }
    }else{
        println!("No value given for Iterations, using default Iterations=1.");
    }

    for it in 0..iterations {
        let mut rng = rand::thread_rng();
        let a: String = (0..rng.gen_range(size_lower_bound..2*size_lower_bound)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
        let b: String = (0..rng.gen_range(size_lower_bound..2*size_lower_bound)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();

        println!("\nStarting sequential version...");
        let start = std::time::Instant::now();
        let seq_res = seq_add_binary_v2(a.clone(), b.clone());
        println!("Sequential done in {:?}!",start.elapsed());
        
        println!("\nStarting parallel version...");
        let start = std::time::Instant::now();
        let par_res = par_add_binary(a.clone(), b.clone(),levels);
        println!("Parallel done in {:?}!",start.elapsed());
        assert!(par_res == seq_res);
    
        let svg_path = format!("Parallel-Add-Binary-#{}.svg",it+1);
        println!("\nSaving Parallel Log svg file {}.",svg_path);
        diam::svg(svg_path, || {
            par_add_binary(a.clone(), b.clone(),levels);
        })
        .expect("Failed saving svg file.");
    }
    

}
