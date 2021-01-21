#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  cargo build 2> output.txt
  ./target/debug/rCC "$input"> tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  rm tmp tmp.s

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}


#test code
#assert $expected $input
assert 0 0

assert 42 42

assert 23 "10 + 1 -2+ 15 -1"

assert 3 "1 * 2 +3/4 +1"

assert 2 "1 * (2 +3) /4 +1"

echo OK