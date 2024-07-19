<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Background

## Esoteric Programming Languages

While the majority of programming languages are designed with the express purpose
of providing an easier and more human-readable interface for communicating with
computers, a minority of languages choose to go a different way.

These esoteric programming languages, or _esolangs_, are designed for different purposes
that include, but are not limited to, Research, Challenge, Art, and Fun. These languages, then,
are not designed to be practical or efficient, but rather to be interesting, challenging and
forcing oneself to think.

The examples of the languages that follow each of the purposes above are:

- Research: [Binary lambda calculus](https://esolangs.org/wiki/Binary_lambda_calculus) is an esolang that is designed to be representable by bits or bytes, encoding lambda calculus.
- Challenge: [Malbolge](https://esolangs.org/wiki/Malbolge) is an esolang designed to be extremely difficult to program in.
- Art: [Piet](https://esolangs.org/wiki/Piet) is an esolang where programs take the shape of bitmaps of abstract art.
- Fun: [LOLCODE](https://esolangs.org/wiki/LOLCODE) is an esolang that purportedly reads as if the lolcats cats were writing code.

## Brainf**k

[Brainf**k](https://esolangs.org/wiki/Brainfuck) is an esoteric programming language that tries to be
minimalist, yet expressive. It happens to be one of the most famous esoteric languages worldwide, not in the
least due to the expletive that is part of its name.

The language was designed by Urban Muller in 1993 with the goal of creating a language for which he could create
the smallest compiler for Amiga OS 2.0. This goal was realized and he managed to write a 240 byte compiler.

## Relationship to Other Esolangs

The design for Brainf**k was inspired by another esolang with the same goal: [FALSE](https://esolangs.org/wiki/FALSE).
The FALSE compiler was 1024 bytes and was implemented for the 68000 assembler.

Another programming language, \\( P'' \\) bears a lot of similarity to Brainf\*\*k. This language was designed by Corrado Böhm
in 1964 as a way to describe a subclass of turing machines. \\( P'' \\) is designed for a left-infinite turing machine and relies
on two symbols and their permutations. The primitive functions for \\( P'' \\) can be mapped directly to those of Brainf**k.

## Turing Completeness

Brainf**k is a [Turing Complete](https://en.wikipedia.org/wiki/Turing_completeness) language under well-defined conditions.
This means that if a certain set of conditions is satisfied, then Brainf**k can be used to compute any computable function.

The conditions for Turing Completeness are:

1. The array is unbounded, or in other words, the memory tape is infinite.
   OR
2. The array is at least three cells long and each cell can store unbounded (infinite) values.


If either of the above mentioned conditions are met, then Brainf**k is a Turing Complete language. The requirements for
certain things to be _unbounded_ may seem to indicate that it is impossible. However, in practice, the _unbounded_ condition is
irrelevant for practical considerations, particularly if you consider modern computers with large memory sizes. In fact, it should
be trivial to implement a VM for Brainf**k that has a practically unbounded memory tape.
