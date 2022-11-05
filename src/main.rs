use rand::Rng;

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

/************* Add Binary *************/


fn seq_add_binary_v1(a: String, b: String) -> String {
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

fn seq_add_binary_v2(a: String, b: String) -> String {
    // Padding the strings with 0s to be the same length with an extra 0
    let (b1,b2) = pad_binaries(a, b);

    b1.chars().rev().into_iter()
    .zip(b2.chars().rev().into_iter())
    .fold(("".to_string(),'0'), |(res,c),(a,b)| {
        let out = add_bits(a,b,c);
        (out.0.to_string() + &res,out.1)
    }).0
}

fn main() {
    let mut rng = rand::thread_rng();
    let a: String = (0..rng.gen_range(150000..200000)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();
    let b: String = (0..rng.gen_range(150000..200000)).into_iter().map(|_| {if rng.gen_bool(0.5) {'0'} else {'1'}}).collect::<String>();

    let start = std::time::Instant::now();
    let seq_res = seq_add_binary_v2(a.clone(), b.clone());
    println!("Sequential done in {:?}!",start.elapsed());
}
