use criterion::{black_box, criterion_group, criterion_main, Criterion};
use db_times_diesel::*;
use db_times_diesel::queries::*;

fn get_one_film_bench (c: &mut Criterion) {
    let connection = &mut establish_connection();
    c.bench_function("get 1 Film", |b| b.iter(|| get_one_film(black_box(connection))));
}

fn get_one_thousand_films_bench (c: &mut Criterion) {
    let connection = &mut establish_connection();
    c.bench_function("get 1000 Films", |b| b.iter(|| get_all_films(black_box(connection))));
}

criterion_group!(benches, get_one_film_bench, get_one_thousand_films_bench);
criterion_main!(benches);
