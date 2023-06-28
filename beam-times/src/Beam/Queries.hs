{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE TypeApplications #-}

module Beam.Queries where

import Beam.DB
import Beam.Types
import Database.Beam
import Database.Beam.MySQL (MySQL)

getAllFilms :: Q MySQL SakilaDB s0 (FilmT (QExpr MySQL s0))
getAllFilms = all_ (_film sakilaDB)

getOneFilm = limit_ 1 $ getAllFilms

getTenFilms = limit_ 10 $ getAllFilms

getHunderAfterHundredFilms = limit_ 100 $ offset_ 100 $ getAllFilms
