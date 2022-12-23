# Simple spreadsheet engine

Simple spreadsheet engine that can handle formulas (`+`, `-`, `*`, `/`, `**`), functions (`IFERROR`, `AVG`, `COUNT`, `MAX`, `MEDIAN`, `MIN`, `STDEV`, `SUM`), cloning/extending formulas and functions, and dependency resolution between cells.

```console
$ poetry run src --debug examples/shopping.csv
Input:
descr  ,amount, unit_price,total_price
Cookies,     4,       2.95,=B2 * C2
Coffee ,     1,=9.60 * 0.8,^
Water  ,     2,       1.20,^
Total  ,      ,           ,=SUM(D2:D4)

D2> B2 * C2
... 4.0 * 2.95
... 11.8

C3> 9.60 * 0.8
... 9.6 * 0.8
... 7.68

D3> ^
D3> B3 * C3
... 1.0 * 7.68
... 7.68

D4> ^
D4> B4 * C4
... 2.0 * 1.2
... 2.4

D5> =SUM(D2:D4)
... SUM(11.8, 7.68, 2.4)
... 21.88

Output:
descr,amount,unit_price,total_price
Cookies,4,2.95,11.8
Coffee,1,7.68,7.68
Water,2,1.20,2.4
Total,,,21.88
```

## Getting started

Install:

```console
make install
```

Run tests:

```console
make test
```
