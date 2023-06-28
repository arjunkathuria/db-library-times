{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE DeriveAnyClass #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE StandaloneDeriving #-}
{-# LANGUAGE TypeFamilies #-}
{-# LANGUAGE TypeSynonymInstances #-}

module Beam.Types where

import Data.Scientific
import Data.Text
import qualified Data.Text.IO as TIO
import Data.Time (LocalTime)
import Database.Beam

data FilmT f = Film
  { _filmFilmId :: Columnar f Word,
    _filmTitle :: Columnar f Text,
    _filmDescription :: Columnar f Text,
    _filmLastUpdate :: Columnar f LocalTime,
    _filmRentalRate :: Columnar f Scientific
    -- _filmLength :: Columnar f Word,
    -- _filmLanguageId :: Columnar f Word
  }
  deriving (Generic)

type Film = FilmT Identity

type FilmPK = PrimaryKey FilmT Identity

deriving instance Show Film

deriving instance Eq Film

instance Beamable FilmT

instance Table FilmT where
  data PrimaryKey FilmT f = FilmPK (Columnar f Word) deriving (Generic, Beamable)
  primaryKey = FilmPK . _filmFilmId

data SakilaDB f = SakilaDB
  { _film :: f (TableEntity FilmT)
  }
  deriving (Generic, Database be)
