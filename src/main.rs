use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Matrix {
    buf: Vec<usize>,
    rows: usize,
    cols: usize
}

impl Matrix {
    fn new (rows: usize, cols: usize) -> Self {
        Matrix {
            buf: vec![0; rows * cols],
            rows,
            cols
        }
    }

    fn set(self: &mut Self, x: usize, y: usize, val: usize) {
        self.buf[x * self.rows + y] = val;
    }

    fn get(self: &Self, x: usize, y: usize) -> usize {
        self.buf[x * self.rows + y]
    }
}

impl Display for Matrix {
    fn fmt(self: &Self, formatter: &mut Formatter) -> std::fmt::Result {
        let mut out = String::from("");  

        for r in 0..self.rows {
            let slice = &self.buf[(self.cols*r)..((r+1)*self.cols)];
            let row = slice.into_iter()
                .map(|v| format!("{:3}", v))
                .fold(String::from(""), |acc, x| format!("{}{},", acc, x));
            out.push_str(&row.to_owned());
            out.push('\n');
        }   
            
        write!(formatter, "{}", out)
    }
}

fn main() {
    println!("RES: {}", lcspd("aggtab", "gxtxayb"));
    println!("RES: {}", lcspd("ANALISE", "ALGORITMOS"));
    println!("RES: {}", lcspd("PROGRAMACAODINAMICA", "TECNICADEOTIMIZACAO"));
    println!("RES: {}", lcspd("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "ZYXWVUTSRQPONMLKJIHGFEDCBA"));
}

fn lcspd(string1: &str, string2: &str) -> usize {
    let mut res = Matrix::new(string2.len() + 1, string1.len() + 1);

    for i in 1..(string1.clone().chars().count() + 1) {
        for j in 1..(string2.clone().chars().count() + 1) {
            res.set(
                i, j, 
                if string1.chars().nth(i - 1) == string2.chars().nth(j - 1) {
                    res.get(i - 1, j - 1) + 1
                } else {
                    max(res.get(i - 1, j), res.get(i, j - 1))
                }
            )
        }
    }

    println!("{}", res);
    
    res.get(string1.len(), string2.len())
}

fn max(v1: usize, v2: usize) -> usize {
    if v1 >= v2 {
        v1
    } else {
        v2
    }
}

#[test]
fn test_lscpd() {
    assert_eq!(lcspd("aggtab", "gxtxayb"), 4);
}