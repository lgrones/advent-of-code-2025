#!/usr/bin/env bash
set -e

DAY="$1"
if [ -z "$DAY" ]; then
	echo "Usage: ./create.sh 12"
	exit 1
fi

DAYNAME="day$(printf "%02d" "$DAY")"

cargo new "$DAYNAME"

echo -e "lib = { path = \"../lib\" }" >>$DAYNAME/Cargo.toml

echo -e "use lib::{StopWatch, read_file};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stopwatch = StopWatch::new();

    let input = read_file(|x| x)?;

    stopwatch.start();

    println!(\"PART 1: {input}\");

    stopwatch.stop();

	stopwatch.start();

    println!(\"PART 2: {input}\");

    stopwatch.stop();

    Ok(())
}" >$DAYNAME/src/main.rs

touch $DAYNAME/src/l.txt
touch $DAYNAME/src/j.txt
