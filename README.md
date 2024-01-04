# Gematr

Simple calculator in Rust. Parse-Tree for expression, Reverse Polish Notation for math.

Example:
```
Input Expression:
	1 + 4 + 86 - 439 + 57^0.5 + min(5^40, -0.00000004859) - 0.38^-64
Syntax Tree:
  BinaryOperator(Minus)
        BinaryOperator(Pow)
              UnaryOperator(Minus)
                    Number("64")
              Number("0.38")
        BinaryOperator(Plus)
              Func("min")
                    UnaryOperator(Minus)
                          Number("0.00000004859")
                    BinaryOperator(Pow)
                          Number("40")
                          Number("5")
              BinaryOperator(Plus)
                    BinaryOperator(Pow)
                          Number("0.5")
                          Number("57")
                    BinaryOperator(Minus)
                          Number("439")
                          BinaryOperator(Plus)
                                Number("86")
                                BinaryOperator(Plus)
                                      Number("4")
                                      Number("1")


Optimized Syntax Tree:
  BinaryOperator(Minus)
        BinaryOperator(Pow)
              Number("-64")
              Number("0.38")
        BinaryOperator(Plus)
              Func("min")
                    Number("-0.00000004859")
                    BinaryOperator(Pow)
                          Number("40")
                          Number("5")
              BinaryOperator(Plus)
                    BinaryOperator(Pow)
                          Number("0.5")
                          Number("57")
                    BinaryOperator(Minus)
                          Number("439")
                          BinaryOperator(Plus)
                                Number("86")
                                BinaryOperator(Plus)
                                      Number("4")
                                      Number("1")


Evaluated: 1 + 4 = 5
Evaluated: 5 + 86 = 91
Evaluated: 91 - 439 = -348
Evaluated: 57 ^ 0.5 = 7.5498344310556950348301013669
Evaluated: 5 ^ 40 = 9094947017729282379150390625
Evaluated: -348 + 7.5498344310556950348301013669 = -340.45016556894430496516989863
Evaluated: min([9094947017729282379150390625, -0.00000004859]) = -0.00000004859
Evaluated: -340.45016556894430496516989863 + -0.00000004859 = -340.45016561753430496516989863
Evaluated: 0.38 ^ -64 = 769230769230769230769230769.23
Evaluated: -340.45016561753430496516989863 - 769230769230769230769230769.23 = -769230769230769230769231109.68
Result: -769230769230769230769231109.68
```
