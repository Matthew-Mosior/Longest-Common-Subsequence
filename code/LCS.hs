{-=Longest Common Subsequence: A Haskell-Based Implementation.=-}
{-=Author: Matthew Mosior=-}
{-=Synposis: This Haskell script will two lexicographic sequences=-}
{-=and return the longest common subsequence between them=-} 
{-=along with the coordinates of each character=-}
{-=in the LCS mapped back to each input string using=-}
{-=a recursive list based implementation.=-}

{-=Imports.=-}

import Data.List
import System.Environment
import System.IO
import Text.PrettyPrint.Boxes

{------------}

{-=Functions Pertaining to Computing the Longest Common Subsequence.=-}
        
--lcslistversion
lcslistversion :: String -> String -> (String,[(Char,Int,Int)])
lcslistversion [] _  = ([],[])
lcslistversion _  [] = ([],[])
lcslistversion xs ys = (firstlist (newlinestripper xs) (newlinestripper ys),secondlist (zip (newlinestripper xs) [0..]) (zip (newlinestripper ys) [0..]))
    where
        --Nested Function Definition.--
        --firstlist
        firstlist :: Eq a => [a] -> [a] -> [a]
        firstlist [] _ = []
        firstlist _ [] = []
        firstlist (x:xs) (y:ys) = if x == y
                                      then x : firstlist xs ys
                                      else longest (firstlist (x:xs) ys) (firstlist xs (y:ys))

        --secondlist
        secondlist :: [(Char,Int)] -> [(Char,Int)] -> [(Char,Int,Int)]
        secondlist [] _                  = []
        secondlist _  []                 = [] 
        secondlist ((a,b):xs) ((c,d):ys) = if a == c
                                               then [(a,b,d)] ++ secondlist xs ys
                                               else longest (secondlist ((a,b):xs) ys) (secondlist xs ((c,d):ys))

        --longest
        longest :: [a] -> [a] -> [a]
        longest xs ys = if length xs > length ys 
                            then xs 
                            else ys

        --newlinestripper
        newlinestripper :: String -> String
        newlinestripper [] = []
        newlinestripper xs = filter (/= '\n') xs
        -------------------------------

--preprintlcs
preprintlcs :: (String,[(Char,Int,Int)]) -> (String,[(String,String,String)])
preprintlcs ([],_) = ([],[])
preprintlcs (_,[]) = ([],[])
preprintlcs (x,y)  = (x,shower y)
    where
        --Nested Function Definition.--
        --shower
        shower :: [(Char,Int,Int)] -> [(String,String,String)]
        shower []           = []
        shower ((a,b,c):xs) = [(show a,show b,show c)] ++ shower xs
        -------------------------------

--prefinalprintlcs
prefinalprintlcs :: (String,[(String,String,String)]) -> (String,[String])
prefinalprintlcs ([],_) = ([],[])
prefinalprintlcs (_,[]) = ([],[])
prefinalprintlcs (x,y) = (x,concatenator y)
    where
        --Nested Function Definition.--
        --concatenator
        concatenator :: [(String,String,String)] -> [String]
        concatenator []           = []
        concatenator ((a,b,c):xs) = [a ++ " " ++ b ++ " " ++ c] ++ concatenator xs
        -------------------------------

--finalprintlcs
finalprintlcs :: (String,[String]) -> [[String]]
finalprintlcs ([],[]) = []
finalprintlcs (x,y)   = map (\z -> [z]) (x : y)

{---------------------------------------------------------------------}


{-=Main Function.=-}

main :: IO ()
main = do
    --Get command line arguments.
    cmdargs <- getArgs
    case cmdargs of
        [] -> error "No input files provided, please provide two input files."
        [arg1,arg2] -> do --Read contents of arg1 and arg2 into String.
                          contents1 <- readFile arg1
                          contents2 <- readFile arg2

                          --Run functions above on contents1 and contents2.
                          let result = finalprintlcs (prefinalprintlcs (preprintlcs (lcslistversion contents1 contents2)))

                          --Write result to output.txt.   
                          writeFile "output.txt" $
                              (render $
                              (hsep 2 left . map (vcat left) . map (map (text)))
                              (transpose result))
        
        _ -> error "More than two input files provided, please provide two input files."

{------------------}
