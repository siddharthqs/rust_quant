// extern crate probability;
// extern crate rand_chacha;
// extern crate rand_pcg;
#![allow(dead_code)]
#![allow(unused_variables)]
use rand;
use rand::{SeedableRng};
use chrono::{Local,DateTime,NaiveDate,NaiveTime};
use rand::distributions::{Standard,Uniform};
use rand::distributions::Distribution;
use rand_distr::StandardNormal;
mod equity;
mod core;
mod utils;
use rand::prelude::*;

use std::{io, thread};
use std::collections::HashMap;
use std::error::Error;
use csv;
//use std::env::{args,Args};
use utils::read_csv;
use utils::RNG;

use std::env::{args, temp_dir};
use rand::Rng;
use equity::blackscholes;
use crate::equity::montecarlo;
use clap::{App,Arg};


fn main() {
    // fn a() -> Result<(), Box<dyn Error>> {
    //     let mut bytes = [0; 400];
    //     let x = vec![1.0,2.0,3.0,4.0,5.0];
    //     LittleEndian::write_f64_into(&x, &mut bytes);
    //     let path = r"D:\siddh\x1.b";
    //     let mut file = File::create(path)?;
    //     file.write_all(&bytes).map_err(|e| e.into())
    //     //fs::write(r"D:\siddh\x.csv", &bytes);
    // }




    let _matches = App::new("qsLib").version("0.1.0").author("Siddharthqs.com")
        .about("Quant Library for retail traders using Rust").get_matches();
    println!("Welcome to Option pricing CLI");
    println!(" Do you want to price option (1) or calculate implied volatility (2)?");

    let mut input = String::new();
    print!("{}", input);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u8 = input.trim().parse::<u8>().unwrap();
    if input == 1 {
        println!("Do you want to use the Black-Sholes (1) or Monte-Carlo (2) option pricing model?");
        let mut model = String::new();
        io::stdin()
            .read_line(&mut model)
            .expect("Failed to read line");
        let model: u8 = model.trim().parse::<u8>().unwrap();
        if model == 1 {
            blackscholes::option_pricing();
        }
        else if model == 2{
            montecarlo::option_pricing();
        }
        else {
            println!("You gave a wrong number! The accepted arguments are 1 and 2.")
        }
    } else if input == 2 {
        blackscholes::implied_volatility();
    } else {
        println!("You gave a wrong number! The accepted arguments are 1 and 2.")
    }




}
