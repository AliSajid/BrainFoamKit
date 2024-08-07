<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Railorad Diagrams

## **Program:**

![program](railroad/program.svg)

```yacc
program  ::= statement*
```

## **Statement:**

![statement](railroad/statement.svg)

```yacc
statement
         ::= increment
           | decrement
           | moveRight
           | moveLeft
           | output
           | input
           | loop
```

referenced by:

* [loop](#loop)
* [program](#program)

## **Loop:**

![loop](railroad/loop.svg)

```yacc
loop     ::= loopStart statement* loopEnd
```

referenced by:

* [statement](#statement)

## Operators

### **Increment:**

![increment](railroad/increment.svg)

```yacc
increment
         ::= '+'
```

referenced by:

* [statement](#statement)

### **Decrement:**

![decrement](railroad/decrement.svg)

```yacc
decrement
         ::= '-'
```

referenced by:

* [statement](#statement)

### **Move Right:**

![moveRight](railroad/moveRight.svg)

```yacc
moveRight
         ::= '>'
```

referenced by:

* [statement](#statement)

### **Move Left:**

![moveLeft](railroad/moveLeft.svg)

```yacc
moveLeft ::= '<'
```

referenced by:

* [statement](#statement)

### **Output:**

![output](railroad/output.svg)

```yacc
output   ::= '.'
```

referenced by:

* [statement](#statement)

### **Input:**

![input](railroad/input.svg)

```yacc
input    ::= ','
```

referenced by:

* [statement](#statement)

### **Loop Start:**

![loopStart](railroad/loopStart.svg)

```yacc
loopStart
         ::= '['
```

referenced by:

* [loop](#loop)

### **Loop End:**

![loopEnd](railroad/loopEnd.svg)

```yacc
loopEnd  ::= ']'
```

referenced by:

* [loop](#loop)


## Acknowledgements

![rr-2.0](railroad/rr-2.0.svg) generated by [RR - Railroad Diagram Generator][RR]

[RR]: http://bottlecaps.de/rr/ui
