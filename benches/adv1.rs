use criterion::{criterion_group, criterion_main, Criterion};
use rand::{
    distributions::Uniform,
    prelude::{SliceRandom, ThreadRng},
    Rng,
};

fn bachi(input: &str) -> u64 {
    let moj = input;

    for i in moj.lines() {
        let result: u64 = i.trim().parse().unwrap();
        for z in moj.lines() {
            let result2: u64 = z.trim().parse().unwrap();
            if result + result2 == 2020 {
                return result * result2;
            }
        }
    }
    panic!()
}

fn feri(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for a in &input {
        for b in &input {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!()
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("aoc-1");

    let mut rng = rand::thread_rng();
    let mut sorted = gen_input(&mut rng);

    sorted.shuffle(&mut rng);
    let input = sorted.join("\n");
    group.bench_function("Bachi", |b| b.iter(|| bachi(&input)));
    group.bench_function("Feri", |b| b.iter(|| feri(&input)));

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);

fn gen_input(rng: &mut ThreadRng) -> Vec<String> {
    let answer = rng.gen_range(1, 2020);
    let answer = vec![answer, 2020 - answer];

    let range = Uniform::new_inclusive(1011, 2019);
    answer
        .into_iter()
        .chain(rng.sample_iter(range))
        .take(200)
        .map(|n| n.to_string())
        .collect()
}
