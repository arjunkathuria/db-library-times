use crate::models::*;
use diesel::prelude::*;

pub fn get_one_film(conn: &mut MysqlConnection) -> Vec<Film> {
    use crate::schema::film::dsl::*;

    let results = film
        //.offset(10)
        .limit(1)
        .select(Film::as_select())
        .load(conn)
        .expect("Error loading Films");

    // println!("Displaying {} posts", results.len());
    // for lol in &results {
    //     println!("-----------\n");
    //     println!("{}", lol.film_id);
    //     println!("{}", lol.title);
    //     println!("{}", lol.description);
    // }

    return results;
}

pub fn get_all_films(conn: &mut MysqlConnection) -> Vec<Film> {
    use crate::schema::film::dsl::*;

    let results = film
        //.offset(10)
        //.limit(1)
        .select(Film::as_select())
        .load(conn)
        .expect("Error loading Films");

    // println!("Displaying {} posts", results.len());
    // for lol in &results {
    //     println!("-----------\n");
    //     println!("{}", lol.film_id);
    //     println!("{}", lol.title);
    //     println!("{}", lol.description);
    // }

    return results;
}
