FROM rust:slim-buster

WORKDIR /app 

COPY ./ .

RUN cargo build

CMD cat input/day1.txt | target/debug/aoc01
