foo();

zero = 0;
hoge = 0;

while (hoge != 2)
hoge = hoge + 1;
a = hoge == 2;

hoge = zero;
for (i = 0; i < 2; i = i + 1)
hoge = hoge + 1;
b = i == 2;
c = hoge == 2;

if (zero == 0)
d = 1;
else
d = 0;

if (zero == 1)
e = 0;
else
e = 1;

if(zero == 1)
f = 0;
else if (zero == 2)
f = 0;
else
f = 1;

if (zero == 1)
g = 0;
else if (zero == 0)
g = 1;
else
g = 0;

h = 0;
if (zero == 0) {
h = h + 2;
h = h - 1;
} else {
h = 0;
}
return h;

a1 = 0;
if (zero != 0){
a1 = 0;
} else {
a1 = 2;
a1 = a1 - 1;
}


return a == b == c == d == e == f == g == h == a1;
