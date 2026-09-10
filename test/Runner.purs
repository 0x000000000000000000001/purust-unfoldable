module Test.Runner where

import Prelude
import Effect (Effect)
import Test.Checks as Checks
import Test.Main as Original

main :: Effect Unit
main = do
  Original.main
  Checks.main
