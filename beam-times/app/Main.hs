{-# LANGUAGE DeriveAnyClass #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE StandaloneDeriving #-}
{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE TypeFamilies #-}
{-# LANGUAGE TypeSynonymInstances #-}

module Main (main) where

import qualified Beam.Connect as BC
import qualified Beam.Queries as BQ
import qualified Data.Text.IO as TIO
import Database.Beam
import Database.Beam.MySQL
import Lib
import qualified MySQLSimple.Connect as MSC
import qualified MySQLSimple.Queries as MSQ

main :: IO ()
main = do
  -- -- Mysql Simple
  msConn <- MSC.connection
  films <- MSQ.getOneFilm msConn
  putStrLn $ show $ films

  -- -- Beam + MySQL-Haskell
  conn <- BC.beamMysqlConnection
  runBeamMySQLDebug TIO.putStrLn conn $ do
    filmsBeam <- runSelectReturningList $ select $ BQ.getOneFilm
    mapM_ (liftIO . putStrLn . show) filmsBeam

  -- irrelevant
  someFunc
