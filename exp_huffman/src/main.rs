use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::fs::{File, read_to_string, write};
use std::io::{Read, Write, BufRead};
use bitvec::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct HuffmanNode {
    frequency: usize,
    character: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_frequency_table(text: &str) -> Vec<(char, usize)> {
    let mut freq_table = std::collections::HashMap::new();
    for c in text.chars() {
        *freq_table.entry(c).or_insert(0) += 1;
    }
    freq_table.into_iter().collect()
}

fn build_huffman_tree(freq_table: &[(char, usize)]) -> HuffmanNode {
    let mut heap = BinaryHeap::new();
    
    for &(c, freq) in freq_table {
        heap.push(HuffmanNode {
            frequency: freq,
            character: Some(c),
            left: None,
            right: None,
        });
    }
    
    while heap.len() > 1 {
        let left = Box::new(heap.pop().unwrap());
        let right = Box::new(heap.pop().unwrap());
        let combined_freq = left.frequency + right.frequency;
        
        heap.push(HuffmanNode {
            frequency: combined_freq,
            character: None,
            left: Some(left),
            right: Some(right),
        });
    }
    
    heap.pop().unwrap()
}

fn build_encoding_table(root: &HuffmanNode) -> std::collections::HashMap<char, BitVec<u8>> {
    let mut encoding_table = std::collections::HashMap::new();
    
    fn traverse(node: &HuffmanNode, current_code: &mut BitVec<u8>, table: &mut std::collections::HashMap<char, BitVec<u8>>) {
        if let Some(c) = node.character {
            table.insert(c, current_code.clone());
        } else {
            if let Some(left) = &node.left {
                current_code.push(false);
                traverse(left, current_code, table);
                current_code.pop();
            }
            if let Some(right) = &node.right {
                current_code.push(true);
                traverse(right, current_code, table);
                current_code.pop();
            }
        }
    }
    
    traverse(root, &mut BitVec::new(), &mut encoding_table);
    encoding_table
}

fn encode_text(text: &str, encoding_table: &std::collections::HashMap<char, BitVec<u8>>) -> BitVec<u8> {
    let mut encoded = BitVec::new();
    for c in text.chars() {
        encoded.extend(encoding_table.get(&c).unwrap());
    }
    encoded
}

fn decode_text(encoded: &BitVec<u8>, root: &HuffmanNode) -> String {
    let mut decoded = String::new();
    let mut current_node = root;
    
    for bit in encoded {
        current_node = if *bit {
            current_node.right.as_ref().unwrap()
        } else {
            current_node.left.as_ref().unwrap()
        };
        
        if let Some(c) = current_node.character {
            decoded.push(c);
            current_node = root;
        }
    }
    
    decoded
}

fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let text = read_to_string(input_path)?;
    let freq_table = build_frequency_table(&text);
    let huffman_tree = build_huffman_tree(&freq_table);
    let encoding_table = build_encoding_table(&huffman_tree);
    let encoded = encode_text(&text, &encoding_table);
    
    let mut file = File::create(output_path)?;
    
    // Write the frequency table
    for (c, freq) in &freq_table {
        write!(file, "{}:{}|", c, freq)?;
        print!("{}:{}|", c, freq);
    }
    writeln!(file)?;
    
    // Write the encoded data
    file.write_all(&encoded.into_vec())?;
    
    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut contents = String::new();

    // First, read the frequency table, which is in UTF-8
    let mut freq_table_part = String::new();
    let mut file_reader = std::io::BufReader::new(&file);
    file_reader.read_line(&mut freq_table_part)?;

    // Parse the frequency table
    let freq_table: Vec<(char, usize)> = freq_table_part
        .trim()
        .split('|')
        .filter(|s| !s.is_empty())  // the last element after delimiter maybe empty e.g. a,b,c,
        .map(|s| {
            let mut parts = s.splitn(2, ':');
            let c = parts.next().unwrap().chars().next().unwrap();
            let freq = parts.next().unwrap().parse().unwrap();
            (c, freq)
        })
        .collect();

    println!("The frequency table is: \n {:?}", freq_table);

    // Now read the remaining file as raw binary data (for encoded bits)
    let mut encoded_data = Vec::new();
    file_reader.read_to_end(&mut encoded_data)?;

    let encoded = BitVec::from_slice(&encoded_data);

    let huffman_tree = build_huffman_tree(&freq_table);
    let decoded = decode_text(&encoded, &huffman_tree);

    write(output_path, decoded)?;

    Ok(())
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 4 {
        eprintln!("Usage: {} <mode> <input_file> <output_file>", args[0]);
        eprintln!("Mode: 'compress' or 'decompress'");
        std::process::exit(1);
    }
    
    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    
    match mode.as_str() {
        "compress" => compress_file(input_file, output_file)?,
        "decompress" => decompress_file(input_file, output_file)?,
        _ => {
            eprintln!("Invalid mode. Use 'compress' or 'decompress'");
            std::process::exit(1);
        }
    }
    
    Ok(())
}


// cargo run -- compress gatsby.txt compressed.bin
// cargo run -- decompress compressed.bin output.txt


// P:1|':8| :437|N:1|B:1|H:2|e:233|h:136|M:1|,:16|v:17|.:16|E:2|g:47|
// :41|w:54|Y:1|y:38|d:103|k:13|A:2|o:144|-:4|i:115|m:41|O:4|z:2|b:27|S:2|G:4|:128|

// [('P', 1), ('\'', 8), (' ', 437), ('N', 1), ('B', 1), ('H', 2), ('e', 233), ('h', 136), ('M', 1), (',', 16), ('v', 17), ('.', 16), ('E', 2), ('g', 47)]



use std::iter::repeat;
use std::iter::zip;


fn add_binary(a: String, b: String) -> String {
        
    let max_len: usize = a.len().max(b.len());    

    let a_padded: Vec<usize> = a.chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(repeat(0))
        .take(max_len)
        .collect();

    let b_padded: Vec<usize> = b.chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .chain(repeat(0))
        .take(max_len)
        .collect();

    println!("a = {:?}", a_padded);
    println!("b = {:?}", b_padded);

    let mut carry = 0;
    let mut result = vec![];
    for (x, y) in zip(a_padded.into_iter(), b_padded.into_iter()) {
        let mut digit_sum = x + y + carry;
        if digit_sum >= 2 {
            digit_sum -= 2;
            carry = 1;
        } else {
            carry = 0;
        }
        result.push(digit_sum);
    }   
    if carry != 0 {
        result.push(1);
    }

    println!("result = {:?}", result);

    let result_str: String = result
        .into_iter()
        .map(|num| num.to_string() )
        .rev()
        .collect();

    result_str
}