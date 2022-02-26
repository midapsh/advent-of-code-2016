# Intro

Problems are taken from this [link](https://adventofcode.com/2016).

Solutions are taken from this [series of videos](https://www.youtube.com/watch?v=AQQTtZCBEdE&list=PLbtjxiXev6lrYBfHl_mhWIPoEV1RAvHhF).

# Running tests

Use the following command

```shell
# For example
cargo test --lib day2
# cargo test --lib day3
# ...
#
# Run specific test (functions)
cargo test --lib day2::tests::test_part_1_dummy # --lib <mod::to::test_function>
# cargo test --lib day3::tests::test_part_1_dummy
# ...
#
# Run test with print
cargo test --lib day2 -- --nocapture
# cargo test --lib day3 -- --nocapture
```

# Running benchmarks

Use the following command

```shell
# For example
cargo bench -- 2-1
# cargo bench -- 2-2
# cargo bench -- 3-1
# ...
```
