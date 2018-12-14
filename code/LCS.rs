/*Longest Common Subsequence: A Rust-Based Implementation.
Author: Matthew Mosior
Synposis: This Rust program will take two lexicographic sequences
and return the longest common subsequence between them
along with the coordinates of each character
in the LCS mapped back to each input string using
a array-based dynamic programming approach.*/


//Imports.

use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;

//////////

//Functions.

//Longest Common Subsequence Function.
fn lcs(string1: String, string2: String) -> (String,Vec<usize>,Vec<usize>)
{
    let total_rows = string1.len() + 1;
    let total_columns = string2.len() + 1; 
    let string1_chars = string1.as_bytes();
    let string2_chars = string2.as_bytes();
 
    //Create nested vectors to hold 2D dynamic programming matrix.
    let mut dynamicprogrammingtable = vec![vec![0; total_columns]; total_rows];
    
    //Saturate the 2D dynamic programming matrix.
    for row in 1..total_rows
    {
        for col in 1..total_columns 
        {
            if string1_chars[row - 1] == string2_chars[col - 1]
            {
                dynamicprogrammingtable[row][col] = dynamicprogrammingtable[row - 1][col - 1] + 1;
            } 
            
            else 
            {
                dynamicprogrammingtable[row][col] = cmp::max(dynamicprogrammingtable[row][col-1], dynamicprogrammingtable[row-1][col]);
            }
        }
    }
  
    //Create vector to hold LCS.
    let mut lcs = Vec::new();
  
    //Create vectors to hold coordinates of LCS in original strings.
    let mut coordinatetable1 : Vec<usize> = Vec::new();
    let mut coordinatetable2 : Vec<usize> = Vec::new();
  
    //Temp variables to hold x and y 2D matrix size.
    let mut x = total_rows - 1;
    let mut y = total_columns - 1;
 
    while x != 0 && y != 0 
    {
        // Check element above is equal
        if dynamicprogrammingtable[x][y] == dynamicprogrammingtable[x - 1][y] 
        {
            x = x - 1;
        }
        // check element to the left is equal
        else if dynamicprogrammingtable[x][y] == dynamicprogrammingtable[x][y - 1] 
        {
            y = y - 1;
        }
        else 
        {
            //Check that the two elements at the respective x,y positions are the same.
            assert_eq!(string1_chars[x-1], string2_chars[y-1]);
            
            //Set char equal to the element.
            let char = string1_chars[x - 1];

            //Add char to the lcs vector.
            lcs.push(char);

            //Add the index of x to coordinatetable1.
            coordinatetable1.push(x);

            //Add the index of y to coordinatetable2.
            coordinatetable2.push(y);

            x = x - 1;
            y = y - 1;
        }
    }

    //Reverse contents of lcs, coordinatetable1, and coordinatetable2.
    lcs.reverse();
    coordinatetable1.reverse();
    coordinatetable2.reverse();

    //Pop the last elements from coordinatetable1 and coordinatetable2.
    coordinatetable1.pop();
    coordinatetable2.pop();

    //Subtract 1 from each element of coordinatetable1 to make the answer zero-indexed.
    for element in coordinatetable1.iter_mut()
    { 
        *element -= 1;
    }
    
    //Subtract 1 from each element of coordinatetable2 to make the answer zero-indexed.
    for element in coordinatetable2.iter_mut()
    { 
        *element -= 1;
    }

    //Return the lcs, coordinatetable1, and coordinatetable2.
    (String::from_utf8(lcs).unwrap(),coordinatetable1,coordinatetable2)
}
//////////////////////////////////////

//Main Function. 
fn main() -> std::io::Result<()>
{
    //Set command line argument.
    let args: Vec<String> = env::args().collect();

    //Match on the number of command line arguments.
    match args.len()
    {
        //Two command lines arguments passed.
        3 =>
        { 
            //Set the two command line arguments as variables.
            let file1 = &args[1];
            let file2 = &args[2];

            //Read file1 and file2 into strings.
            let buf1 = File::open(file1)?;
            let mut buf_reader1 = BufReader::new(buf1);
            let mut contents1 = String::new();
            buf_reader1.read_to_string(&mut contents1)?;

            let buf2 = File::open(file2)?;
            let mut buf_reader2 = BufReader::new(buf2);
            let mut contents2 = String::new();
            buf_reader2.read_to_string(&mut contents2)?;
            ////////////////////////////////////

            //Run the lcs function on contents1 and contents2.
            let runlcs  = lcs(contents1,contents2);
            let (a,b,c) = runlcs; 
            //////////////////////////////////////////////////
    
            //Print the result of runlcs to output.txt.
            let mut output_buffer = File::create("output.txt")?;
            
            //Write the first element of runlcs to output.txt.
            write!(output_buffer,"{}",a);

            //Write the second element of runlcs to output.txt.
            for data in &b
            {
                write!(output_buffer,"{} ",data);
            }
           
            //Insert a newline.
            output_buffer.write_all(b"\n");
            
            //Write the third element of runlcs to output.txt.
            for data in &c
            {
                write!(output_buffer,"{} ",data);
            }

            //Insert a newline.
            output_buffer.write_all(b"\n");
            ///////////////////////////////////////////
        }

        //All other cases.
        _ =>
        {
            println!("Please provide two text files.");
        }
    }

    Ok(())
}
////////////////

////////////
