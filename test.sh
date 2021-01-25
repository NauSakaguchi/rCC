#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  if [ "$input" = "test.c" ]; then
    ./target/debug/rCC > tmp.s
  else
    ./target/debug/rCC "$input"> tmp.s
  fi

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

cargo build 2> output.txt

#test code
#assert $expected $input
assert 0 "0;"

assert 42 "42;"

assert 23 "10 + 1 -2+ 15 -1;"

assert 10 "2 * 3 + 6 - 4 / 2;"
assert 3 "1 * 2 +3/4 +1;"
assert 2 "1 * (2 +3) /4 +1;"
assert 1 "1 * -(-3 +1) /4 +1;"
assert 3 "1 * -3 * (-1);"

assert 0 "10 == 20;"
assert 1 "10 == 10;"
assert 1 "10 != 20;"
assert 0 "10 != 10;"
assert 0 "10 > 10;"
assert 0 "10 < 10;"
assert 1 "10 <= 20;"
assert 1 "10 <= 10;"
assert 0 "10 >= 20;"
assert 1 "10 >= 10;"

assert 1 "foo = 10; foo == 10;"
assert 0 "foo = 10; foo < 10;"
assert 30 "foo = 10; bar = 20; foo + bar;"
assert 30 "foo = 10;bar = 20; foo + bar;"
assert 4 "return 4;"
assert 4 "return 4; return 5;"
assert 30 "foo = 10; bar = 20; return foo + bar;"
assert 30 "foo = 10; bar = 20; return foo + bar; return foo;"
assert 1 "test.c"

echo OK


