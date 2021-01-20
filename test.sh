#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.s
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
assert 0 0

assert 42 42

echo OK