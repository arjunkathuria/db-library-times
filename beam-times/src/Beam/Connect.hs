{-# LANGUAGE OverloadedStrings #-}

module Beam.Connect (beamMysqlConnection, MSB.MySQLConn, closeConnection) where

import qualified Database.MySQL.Base as MSB

beamMysqlConnection :: IO MSB.MySQLConn
beamMysqlConnection =
  MSB.connect $
    MSB.defaultConnectInfo
      { MSB.ciDatabase = "sakila",
        MSB.ciUser = "user_1",
        MSB.ciPassword = "user1_password"
      }

closeConnection :: MSB.MySQLConn -> IO ()
closeConnection = MSB.close
