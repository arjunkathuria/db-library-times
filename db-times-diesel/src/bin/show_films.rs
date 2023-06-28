use self::models::*;
use db_times_diesel::*;
use diesel::prelude::*;

fn main() {
    use self::schema::film::dsl::*;

    let connection = &mut establish_connection();
    let results = film
        //.offset(10)
        //.limit(10)
        .select(Film::as_select())
        .load(connection)
        .expect("Error loading films");

    println!("Displaying {} posts", results.len());
    for hi in results {
        println!("-----------\n");
        println!("{}", hi.film_id);
        println!("{}", hi.title);
        println!("{}", hi.description);
    }
}
