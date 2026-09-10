module Test.Checks where

import Prelude

import Data.Array as Array
import Data.Array.NonEmpty as NonEmpty
import Data.Foldable (foldl)
import Data.Maybe (Maybe(..))
import Data.Tuple (Tuple(..))
import Data.Unfoldable as U
import Data.Unfoldable1 as U1
import Effect (Effect)
import Effect.Console (log)
import Effect.Ref as Ref
import Test.Assert (assert)

emptyAndTerminal :: Effect Unit
emptyAndTerminal = do
  log "Check empty and terminal steps"
  assert $ U.unfoldr (\_ -> Nothing) unit == ([] :: Array Int)
  assert $ U.unfoldr (\done -> if done then Nothing else Just (Tuple "last" true)) false == ["last"]
  assert $ U1.unfoldr1 (\_ -> Tuple "last" Nothing) unit == ["last"]


structuredValues :: Effect Unit
structuredValues = do
  log "Check structured states and payloads"
  let
    structured = U.unfoldr (\(Tuple n total) ->
      if n == 0 then Nothing
      else Just (Tuple { index: n, total } (Tuple (n - 1) (total + n)))) (Tuple 3 10)
    nonEmpty = U1.unfoldr1 (\{ remaining, value } ->
      Tuple (Tuple value [remaining])
        (if remaining == 0 then Nothing
         else Just { remaining: remaining - 1, value: value + 2 }))
      { remaining: 2, value: 5 }
  assert $ structured == [{ index: 3, total: 10 }, { index: 2, total: 13 }, { index: 1, total: 15 }]
  assert $ nonEmpty == [Tuple 5 [2], Tuple 7 [1], Tuple 9 [0]]


producedFunctions :: Effect Unit
producedFunctions = do
  log "Check produced functions"
  let
    functions = U.unfoldr (\n ->
      if n == 0 then Nothing
      else Just (Tuple (\x -> x + n) (n - 1))) 3
    nonEmptyFunctions = U1.unfoldr1 (\n ->
      Tuple (\x -> x * n) (if n == 1 then Nothing else Just (n - 1))) 3
  assert $ map (_ $ 10) functions == [13, 12, 11]
  assert $ map (_ $ 10) nonEmptyFunctions == [30, 20, 10]


countBoundaries :: Effect Unit
countBoundaries = do
  log "Check count boundaries and descending ranges"
  assert $ U.replicate (-3) "x" == []
  assert $ U1.replicate1 (-3) "x" == ["x"]
  assert $ U1.iterateN 0 (_ + 1) 42 == [42]
  assert $ U1.iterateN (-3) (_ + 1) 42 == [42]
  assert $ U1.range 2 (-2) == [2, 1, 0, -1, -2]


maybeInstances :: Effect Unit
maybeInstances = do
  log "Check Maybe instances"
  assert $ (U.none :: Maybe Int) == Nothing
  assert $ (U.fromMaybe (Just 42) :: Maybe Int) == Just 42
  assert $ (U.unfoldr (\n -> Just (Tuple n (n + 1))) 42 :: Maybe Int) == Just 42
  assert $ (U1.unfoldr1 (\n -> Tuple n (Just (n + 1))) 42 :: Maybe Int) == Just 42


deferredEffects :: Effect Unit
deferredEffects = do
  log "Check deferred effects and replay"
  counter <- Ref.new 0
  let
    next = Ref.modify (_ + 1) counter
    action = U.replicateA 3 next :: Effect (Array Int)
    emptyAction = U.replicateA 0 next :: Effect (Array Int)
  before <- Ref.read counter
  assert $ before == 0
  empty <- emptyAction
  afterEmpty <- Ref.read counter
  assert $ empty == [] && afterEmpty == 0
  first <- action
  second <- action
  assert $ first == [1, 2, 3] && second == [4, 5, 6]
  nonEmptyAction <- U1.replicate1A 0 next
  assert $ NonEmpty.toArray nonEmptyAction == [7]
  threeActions <- U1.replicate1A 3 next
  assert $ NonEmpty.toArray threeActions == [8, 9, 10]


largeUnfolds :: Effect Unit
largeUnfolds = do
  log "Check large iterative unfolds"
  let
    values = U.unfoldr (\n -> if n == 10000 then Nothing else Just (Tuple n (n + 1))) 0 :: Array Int
    nonEmptyValues = U1.range 0 9999 :: Array Int
  assert $ Array.length values == 10000
  assert $ Array.length nonEmptyValues == 10000
  assert $ foldl (+) 0 values == 49995000
  assert $ values == nonEmptyValues


main :: Effect Unit
main = do
  emptyAndTerminal
  structuredValues
  producedFunctions
  countBoundaries
  maybeInstances
  deferredEffects
  largeUnfolds
  log "Unfoldable integration checks passed."
