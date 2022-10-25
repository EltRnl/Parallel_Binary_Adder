use std::{char::from_digit, mem::take};

use rand::Rng;
use rayon::{string, result};


fn xor(c1: char, c2: char) -> char{
    if c1=='0' {return c2;}
    if c2=='0' {return '1';}
    return '0';
}

fn and(c1: char, c2: char) -> char{
    if c1=='1' && c2=='1' {return '1';}
    return '0';
}

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

fn seq_add_binary(a: String, b: String) -> String {
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

fn main() {
    let mut rng = rand::thread_rng();
    let a: String = (0..rng.gen_range(100..200)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
    //let a: String = (0..).into_iter().map(|_| '1').take(1).collect::<String>();
    let b: String = (0..rng.gen_range(100..200)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
    //let b: String = (0..rng.gen_range(100..200)).into_iter().map(|_| '1').collect::<String>();

    
    let result = seq_add_binary(a.clone(), b.clone());
    println!("A = {} \n\nB = {} \n\nA + B = {}",a,b,result);
}
