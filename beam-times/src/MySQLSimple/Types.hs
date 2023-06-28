{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}

module MySQLSimple.Types where

import Data.Text
import Data.Time
import Database.MySQL.Simple.QueryResults
import Database.MySQL.Simple.Result
import GHC.Generics (Generic)

data Film = Film
  { filmId :: Word,
    title :: Text,
    description :: Text,
    last_update :: UTCTime,
    rental_rate :: Double
  }
  deriving (Show, Generic)

instance QueryResults Film where
  -- convertResults [fa,fb,fc] [va,vb,vc] = Film a b c Nothing
  --     where !a = convert fa va
  --           !b = convert fb vb
  --           !c = convert fc vc
  -- convertResults [fa,fb,fc,fd] [va,vb,vc,vd] = Film a b c d
  --     where !a = convert fa va
  --           !b = convert fb vb
  --           !c = convert fc vc
  --           !d = convert fd vd

  convertResults [fa, fb, fc, fd, fe] [va, vb, vc, vd, ve] = Film a b c d e
    where
      !a = convert fa va
      !b = convert fb vb
      !c = convert fc vc
      !d = convert fd vd
      !e = convert fe ve
  convertResults fs vs = convertError fs vs 5
