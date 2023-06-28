{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE StandaloneDeriving #-}
{-# LANGUAGE TypeApplications #-}

module Main where

import qualified Beam.Connect as BC
import qualified Beam.Queries as BQ
import qualified Beam.Types as BT
import Control.DeepSeq
import Criterion
import Criterion.Main
import Database.Beam
import Database.Beam.MySQL
import Database.MySQL.Simple (Connection)
import qualified MySQLSimple.Connect as MSC
import qualified MySQLSimple.Queries as MSQ
import qualified MySQLSimple.Types as MST

instance NFData BC.MySQLConn where
  rnf = rwhnf

instance NFData Connection where
  rnf = rwhnf

instance NFData (BT.FilmT Identity)

instance NFData (MST.Film)

setupEnvBeam :: IO BC.MySQLConn
setupEnvBeam = do
  conn <- BC.beamMysqlConnection
  pure conn

setupEnvMySQLSimple :: IO Connection
setupEnvMySQLSimple = do
  conn <- MSC.connection
  pure conn

main :: IO ()
main =
  defaultMain
    [ bgroup
        "BEAM//"
        [ ( envWithCleanup
              setupEnvBeam
              (\conn -> BC.closeConnection conn)
              $ \conn -> do
                bench "get one Film - 5 cols" $ nfIO $ do
                  runBeamMySQL conn $ do
                    films <- runSelectReturningList $ select BQ.getOneFilm
                    -- liftIO $ print films
                    pure films
          ),
          ( envWithCleanup
              setupEnvBeam
              (\conn -> BC.closeConnection conn)
              $ \conn -> do
                bench "get ten Films - 5 cols" $ nfIO $ do
                  runBeamMySQL conn $ do
                    films <- runSelectReturningList $ select BQ.getTenFilms
                    -- liftIO $ print films
                    pure films
          ),
          ( envWithCleanup
              setupEnvBeam
              (\conn -> BC.closeConnection conn)
              $ \conn -> do
                bench "get 100 Films after the first 100 - 5 cols" $ nfIO $ do
                  runBeamMySQL conn $ do
                    films <- runSelectReturningList $ select BQ.getHunderAfterHundredFilms
                    -- liftIO $ print films
                    pure films
          ),
          ( envWithCleanup
              setupEnvBeam
              (\conn -> BC.closeConnection conn)
              $ \conn -> do
                bench "get one thousand Films - 5 cols" $ nfIO $ do
                  runBeamMySQL conn $ do
                    films <- runSelectReturningList $ select BQ.getAllFilms
                    -- liftIO $ print films
                    pure films
          )
        ],
      bgroup
        "MySQL Simple//"
        [ ( envWithCleanup
              setupEnvMySQLSimple
              (\conn -> MSC.closeConnection conn)
              $ \conn -> do
                bench "get one Film - 5 cols" $ nfIO $ do
                  film <- MSQ.getOneFilm conn
                  -- print film
                  pure film
          ),
          ( envWithCleanup
              setupEnvMySQLSimple
              (\conn -> MSC.closeConnection conn)
              $ \conn -> do
                bench "get Ten Films - 5 cols" $ nfIO $ do
                  films <- MSQ.getTenFilms conn
                  -- print films
                  pure films
          ),
          ( envWithCleanup
              setupEnvMySQLSimple
              (\conn -> MSC.closeConnection conn)
              $ \conn -> do
                bench "get Hundred Films - 5 cols" $ nfIO $ do
                  films <- MSQ.getHundredFilms conn
                  -- print films
                  pure films
          ),
          ( envWithCleanup
              setupEnvMySQLSimple
              (\conn -> MSC.closeConnection conn)
              $ \conn -> do
                bench "get one thousand Films - 5 cols" $ nfIO $ do
                  films <- MSQ.getAllFilms conn
                  -- print films
                  pure films
          )
        ]
    ]
