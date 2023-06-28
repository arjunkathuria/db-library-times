module MySQLSimple.Connect (connection, closeConnection) where

import Database.MySQL.Simple (Connection)
import qualified Database.MySQL.Simple as MS

connection :: IO Connection
connection =
  MS.connect $
    MS.defaultConnectInfo
      { MS.connectUser = "user_1",
        MS.connectPassword = "user1_password",
        MS.connectDatabase = "sakila"
      }

closeConnection :: Connection -> IO ()
closeConnection = MS.close
