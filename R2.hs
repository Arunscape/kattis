solve :: [Int] -> Int
solve [r1, s] = 2 * s - r1 

main :: IO()
main = interact $ show . solve . map read . words
