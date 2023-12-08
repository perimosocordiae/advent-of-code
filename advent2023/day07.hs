-- Usage: runghc day07.hs
import Data.List
import Text.Printf (printf)

main = do
  text <- readFile "inputs/07.full"
  printf "Part 1: %d\n" $ part1 (lines text)
  printf "Part 2: %d\n" $ part2 (lines text)

part1 :: [String] -> Int
part1 lines = totalWinnings $ map handleLine lines

part2 :: [String] -> Int
part2 lines = totalWinnings $ map handleLine' lines

totalWinnings :: [((Int, [Int]), Int)] -> Int
totalWinnings hands = sum $ map (\(rank, (_, wager)) -> rank * wager) ranking
  where
    ranking = zip [1 ..] $ sort hands

handleLine :: String -> ((Int, [Int]), Int)
handleLine line = (score, wager)
  where
    parts = words line
    wager = read (parts !! 1) :: Int
    score = scoreHand (head parts)

handleLine' :: String -> ((Int, [Int]), Int)
handleLine' line = (score, wager)
  where
    parts = words line
    wager = read (parts !! 1) :: Int
    score = scoreHand' (head parts)

data HandType = HighCard | Pair | TwoPairs | ThreeOfAKind | FullHouse | FourOfAKind | FiveOfAKind
  deriving (Show, Eq)

scoreHandType :: HandType -> Int
scoreHandType handType = case handType of
  HighCard -> 0
  Pair -> 1
  TwoPairs -> 2
  ThreeOfAKind -> 3
  FullHouse -> 4
  FourOfAKind -> 5
  FiveOfAKind -> 6

-- hand is a 5-character string representing a poker hand w/o suits
scoreHand :: String -> (Int, [Int])
scoreHand hand = (typeScore, highCardScores)
  where
    groupSizes = sort $ map length $ group $ sort hand
    handType = case groupSizes of
      [5] -> FiveOfAKind
      [1, 4] -> FourOfAKind
      [2, 3] -> FullHouse
      [1, 1, 3] -> ThreeOfAKind
      [1, 2, 2] -> TwoPairs
      [1, 1, 1, 2] -> Pair
      [1, 1, 1, 1, 1] -> HighCard
    typeScore = scoreHandType handType
    highCardScores = map scoreCard hand

-- Same as above, but with J as a wild card
scoreHand' :: String -> (Int, [Int])
scoreHand' hand = (typeScore, highCardScores)
  where
    groups = group $ sort hand
    groupSizes = sort $ map length groups
    numWilds = maybe 0 length $ find (\g -> head g == 'J') groups
    handType = case (numWilds, groupSizes) of
      (_, [5]) -> FiveOfAKind
      --
      (0, [1, 4]) -> FourOfAKind
      (_, [1, 4]) -> FiveOfAKind
      --
      (0, [2, 3]) -> FullHouse
      (_, [2, 3]) -> FiveOfAKind
      --
      (0, [1, 1, 3]) -> ThreeOfAKind
      (1, [1, 1, 3]) -> FourOfAKind
      (3, [1, 1, 3]) -> FourOfAKind
      --
      (0, [1, 2, 2]) -> TwoPairs
      (1, [1, 2, 2]) -> FullHouse
      (2, [1, 2, 2]) -> FourOfAKind
      --
      (0, [1, 1, 1, 2]) -> Pair
      (_, [1, 1, 1, 2]) -> ThreeOfAKind
      --
      (0, [1, 1, 1, 1, 1]) -> HighCard
      (1, [1, 1, 1, 1, 1]) -> Pair
    typeScore = scoreHandType handType
    highCardScores = map scoreCard' hand

-- card ordering: 2 3 4 5 6 7 8 9 T J Q K A
scoreCard :: Char -> Int
scoreCard card = case card of
  'A' -> 14
  'K' -> 13
  'Q' -> 12
  'J' -> 11
  'T' -> 10
  _ -> read [card] :: Int

-- card ordering: J 2 3 4 5 6 7 8 9 T Q K A
scoreCard' :: Char -> Int
scoreCard' card = case card of
  'A' -> 13
  'K' -> 12
  'Q' -> 11
  'J' -> 1
  'T' -> 10
  _ -> read [card] :: Int