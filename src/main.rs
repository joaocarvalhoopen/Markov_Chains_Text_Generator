/// Name: Markov Chains Text Generator   
///
/// Date: 2021.10.23
///
/// Author: Joao Nuno Carvalho
///
/// Description: This program extract the n-grams probabilistic Markov Chaines model
///              from a source text and generates random text from the model, with
///              same statistical proprieties. 
///
/// License: The License is MIT Open Source.
/// 
/// This code is a modified re-implementation in Rust of the code in Javascript
/// from the Coding Train episode (See the project page for all the references):
/// 
/// The Coding Train
/// Session 6: N-Grams and Markov Chains - Programming with Text
/// https://www.youtube.com/playlist?list=PLRqwX-V7Uu6ah9Oqs_BFQIbGIn1XynsVT
///
///
/// Notes: This is a experimental project just for fun not a complete program,
///        so experiment with it.
///        To put a new text just add the test to the string of last function
///        get_dostoyevsky(), change the starting word and the n-grams length
///        inside generate_dostoyevsky().
///        The results aren't of the quality of GPT3 but the training of
///        the model is instant and doesn't cost millions of dollars :-)
///        
/// TODO: 
///       - Do the Command Line parser by hand or using the Clap lib.
///       - Allow to generate at the same time separate models from more then
///         one book, and then sample from the different models por each
///         n-gram at a specific probability, example 20 % from one book,
///         30 % from other, and 50 % from other, while generating only one
///         text string with n characters.
/// 
/// How to:
///     To only build do: cargo build
///
///     To run do: cargo run --release
///
///     To run the tests do: cargo test --release
///
/// 
/// #[inline(always)]
/// 

extern crate rand;

use std::collections::HashMap;
use std::process;

use rand::{thread_rng, Rng};

fn main() {
    println!("************************************");
    println!("*   Markov Chains Text Generator!  *");
    println!("************************************");

    generate_simple_text();
    // generate_dostoyevsky();
}

fn generate_simple_text() {
    // let text = "the theremin is theirs, ok? yes, it is. this is a theremin.".to_string();
        
    let text = "The unicorn is a legendary creature that has been described since antiquity \
    as a beast with a large, pointed, spiraling horn projecting from its forehead. The unicorn \
    was depicted in ancient seals of the Indus Valley Civilization and was mentioned by the \
    ancient Greeks in accounts of natural history by various writers, including Ctesias, Strabo, \
    Pliny the Younger, and Aelian.[1] The Bible also describes an animal, the re'em, which some\
    translations have erroneously rendered with the word unicorn.[1] In European folklore, the \
    unicorn is often depicted as a white horse-like or goat-like animal with a long horn and cloven \
    hooves (sometimes a goat's beard). In the Middle Ages and Renaissance, it was commonly described \
    as an extremely wild woodland creature, a symbol of purity and grace, which could only be \
    captured by a virgin. In the encyclopedias its horn was said to have the power to render poisoned \
    water potable and to heal sickness. In medieval and Renaissance times, the tusk of the narwhal \
    was sometimes sold as unicorn horn.".to_string();


    let order = 4; // 3
    let mut m_model = MarkovModel::new(order);
    m_model.generate_model(& text);
    
    let start_text = "the ".to_string();
    let gen_text_len: usize = 200; // 50;
    println!("Start text: {}\n\n", start_text);
    let gen_text = m_model.generate_text(& start_text, gen_text_len);
    println!("{}", gen_text); 
}

fn generate_dostoyevsky() {
    let text = get_dostoyevsky();

    let order = 6; // 3
    let mut m_model = MarkovModel::new(order);
    m_model.generate_model(& text);
    
    let start_text = "someon".to_string();
    let gen_text_len: usize = 100; // 50;
    println!("Start text: {}\n\n", start_text);
    let gen_text = m_model.generate_text(& start_text, gen_text_len);
    println!("{}", gen_text); 
}

//***********************************************
// Code
//

struct MarkovModel {
    order: usize,
    ngrams: HashMap<String,Vec<char>>,
}

impl MarkovModel {
    fn new(order: usize) -> Self {
        MarkovModel {
            order,
            ngrams: HashMap::new(),
        }
    }

    fn generate_model(& mut self, text: & String) {
        let mut vec_tmp: Vec<char> = Vec::new();
        let mut str_tmp = String::new();
        for c in text.chars() {
            vec_tmp.push(c);
            if vec_tmp.len() == self.order + 1 {
                // Note: This takes the first element and copies the all buffer,
                //       but the buffer is small.
                str_tmp.clear();
                str_tmp.extend(vec_tmp[0..self.order].iter());
                let next_char = vec_tmp[self.order].clone();
                // println!("{} -> {}", str_tmp, next_char);
                match self.ngrams.get_mut(& str_tmp) {
                    None => { self.ngrams.insert(str_tmp.clone(), vec![next_char]);}
                    Some(inner_vec) => { inner_vec.push(next_char);}
                }
                vec_tmp.remove(0);
            }
        }
        // println!("\nn_grams: {:?}", self.ngrams);
    }

    fn generate_text(& self, start_text: & String, gen_text_len: usize) -> String {
        let mut gen_text = String::with_capacity(gen_text_len);
        gen_text.push_str(start_text);
        let num_chars = gen_text_len - start_text.len();
        let mut curr_gram = start_text.clone();
        let mut rng = thread_rng();

        for _i in 0..num_chars {

            match self.ngrams.get(& curr_gram) {
                None => {
                        println!("Error: Invalid starting n-gram text!");
                        process::exit(0);
                    },
                Some(inner_vec) => {
                        let index = rng.gen_range(0..inner_vec.len());
                        let c = inner_vec[index];
                        gen_text.push(c);
                        let v = curr_gram.chars().skip(1).collect::<Vec<char>>();
                        curr_gram = v.iter().collect();
                        curr_gram.push(c);
                        // println!("{}", curr_gram);
                    },
            }
        }
        
        gen_text
    }

}

// Dostoyevsky - The Idiot - Segment of the book in public domain from Project Guttemberg.

fn get_dostoyevsky() -> String {

r####"
Put the text here.
"####.to_string()    

}

