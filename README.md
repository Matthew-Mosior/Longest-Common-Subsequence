# Longest-Common-Subsequence: A comparison between Haskell and Rust implementations of the longest common subsequence (LCS) problem.

## Introduction

The longest common subsequence (LCS) problem has been around for quite sometime, and has seen interest from different fields, such as computer science and bioinformatics.  **LCS.hs** (written in Haskell) and **LCS.rs** (written in Rust) are two differing implementations of the longest common subsequence problem.  The recursive nature of **LCS.hs** and the iterative nature of **LCS.rs** demonstrate the constrasting programming paradigms and differing solutions, while achieving the same result.

## Usage

**LCS.hs** and **LCS.rs** are very easy to use.  
Both **LCS.hs** and **LCS.rs** take two command line arguments: 
- A text file containing a sequence of characters.
- Another text file containing a sequence of characters.

They both generate **output.txt** in the current directory.

The following is an example usage of **LCS.hs**

```
$ ./LCS_haskell 
```

The **LCS_haskell** and **LCS_rust** executables can be found [here](https://github.com/Matthew-Mosior/Longest-Common-Subsequence/tree/master/bin), if you prefer not to compile the source code yourself.

## Example

Two example files would be the following:

_example1.txt_

```
thisisatest
```

_example2.txt_

```
testing123testing
```

Given the input files above, **LCS.hs** output will look like:

_output_haskell.txt_

```
tsitest  
't' 0 0  
's' 3 2  
'i' 4 4  
't' 7 10 
'e' 8 11 
's' 9 12 
't' 10 13
```

Similarly, **LCS.rs** will look like:

_output_rust.txt_

```
tsitest
0 3 4 7 8 9 10
0 2 4 10 11 12 13
```

These files can be found [here](https://github.com/Matthew-Mosior/Longest-Common-Subsequence/tree/master/examples) as well.
