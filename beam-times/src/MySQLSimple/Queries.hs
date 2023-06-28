{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE OverloadedStrings #-}

module MySQLSimple.Queries where

import Database.MySQL.Simple (Connection, query, query_)
import MySQLSimple.Types

getAllFilms :: Connection -> IO [Film]
getAllFilms conn = query_ conn "select film_id,title,description,last_update,rental_rate from film"

getOneFilm :: Connection -> IO [Film]
getOneFilm conn = query_ conn "select film_id,title,description,last_update,rental_rate from film limit 1;"

getTenFilms :: Connection -> IO [Film]
getTenFilms conn = query_ conn "select film_id,title,description,last_update,rental_rate from film limit 10;"

getHundredFilms :: Connection -> IO [Film]
getHundredFilms conn = query_ conn "select film_id,title,description,last_update,rental_rate from film limit 100 offset 100;"

testQuery :: Connection -> Int -> IO [Film]
testQuery conn id = query conn "select film_id,title,description,last_update,rental_rate from film where film_id=?;" [id]
