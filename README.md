# Longest-Common-Subsequence: A comparison between Haskell and Rust implementations of the longest common subsequence (LCS) problem.

## Introduction

The longest common subsequence (LCS) problem has been around for quite sometime, and has seen interest from different fields, such as computer science and bioinformatics.  **LCS.hs** (written in Haskell) and **LCS.rs** (written in Rust) are two differing implementations of the longest common subsequence problem.  The recursive nature of **LCS.hs** and the iterative nature of **LCS.rs** demonstrate the contrasting programming paradigms and differing solutions, while achieving the same result.

## Usage

**LCS.hs** and **LCS.rs** are very easy to use.  
Both **LCS.hs** and **LCS.rs** take two command line arguments: 
- A text file containing a sequence of characters.
- Another text file containing a sequence of characters.

They both generate **output.txt** in the current directory.

The only difference between the two implementations is that the Haskell implementation utilizes recursion to calculate the LCS, so it won't capture information regarding the LCS edit graph (2D dynamic programming grid).

## Haskell Compilation

To actually run **LCS.hs** and **LCS.rs**, they need to be compiled first.

To compile **LCS.hs**, you will need a haskell compiler, such as GHC, which can be downloaded [here](https://www.haskell.org/downloads).

Once you have GHC installed, make sure you have the libraries required by **LCS.hs** locally.  If not, use the following command to download them:

```
$ cabal update
$ cabal install [library-name]
```

Then, to compile **LCS.hs** using GHC, use the following command:

```
$ ghc -o LCS_haskell LCS.hs
```

This command will output _LCS_haskell_.

The following is an example usage of _LCS_haskell_:

```
$ ./LCS_haskell example1.txt example2.txt
```

## Rust Compilation

To compile **LCS.rs**, you will need the rust compiler, rustc, which can be downloaded [here](https://www.rust-lang.org/learn/get-started).

Then, to compile **LCS.rs** using rustc, use the following command:

```
$ rustc -o LCS_rust LCS.rs
```

This will output _LCS_rust_.

The following is an example usage of _LCS_rust_:

```
$ ./LCS_rust example1.txt example2.txt
```

## Pre-compiled Binaries

The _LCS_haskell_ and _LCS_rust_ binaries can be found [here](https://github.com/Matthew-Mosior/Longest-Common-Subsequence/tree/master/bin), if you prefer not to compile the source code yourself.

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
LCS:                    
tsitest
                
Coordinates of elements:
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
LCS:
tsitest

Coordinates of elements in "../Code/Examples/example1.txt":
0 3 4 7 8 9 10 
Coordinates of elements in "../Code/Examples/example2.txt":
0 2 4 10 11 12 13 

Dynamic programming grid:
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
0 1 1 1 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 
0 1 1 2 2 2 2 2 2 2 2 2 2 3 3 3 3 3 3 
0 1 1 2 2 3 3 3 3 3 3 3 3 3 3 4 4 4 4 
0 1 1 2 2 3 3 3 3 3 3 3 3 4 4 4 4 4 4 
0 1 1 2 2 3 3 3 3 3 3 3 3 4 4 4 4 4 4 
0 1 1 2 3 3 3 3 3 3 3 4 4 4 5 5 5 5 5 
0 1 2 2 3 3 3 3 3 3 3 4 5 5 5 5 5 5 5 
0 1 2 3 3 3 3 3 3 3 3 4 5 6 6 6 6 6 6 
0 1 2 3 4 4 4 4 4 4 4 4 5 6 7 7 7 7 7 
0 1 2 3 4 4 4 4 4 4 4 4 5 6 7 7 7 7 8 
```

These files can be found [here](https://github.com/Matthew-Mosior/Longest-Common-Subsequence/tree/master/examples) as well.

## Credits

Both the Haskell and Rust implementations were added December 2018.

Author: [Matthew Mosior](https://github.com/Matthew-Mosior)

