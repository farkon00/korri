# Korri Documentation

Here you will find a full documentation of the __Korri__ language.

## Simple Mathematics

The basic operations `+`, `-`, `/`, `*`, and `^` are all present in __Korri__; add, subtract, divide\*, multiply, and exponential respectively.

\* Divide here refers to integer division. For example, `5 / 2` will return `2` rather than `2.5`. For floating point division, see [Floating Point Arithmetic](#floating-point-arithmetic).

## Floating Point Arithmetic

Floating point arithmetic is nothing special in Korri, but some operations require a different operator. For example, to do floating point division, `//` must be used rather than just `/`.

Example:

```
5 / 2 = 2
5 // 2 = 2.5
```

A full list of the floating point equivalent of integer operations can be found below:

| Floating Point | Integer | Example         |
|----------------|---------|-----------------|
| //             | /       | 5 // 2 = 2.5    |
