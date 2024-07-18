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

- Research: [Binary lambda calculus](https://esolangs.org/wiki/Binary_lambda_calculus)
- Challenge: [Malbolge](https://esolangs.org/wiki/Malbolge)
- Art: [Piet](https://esolangs.org/wiki/Piet)
- Fun: [LOLCODE](https://esolangs.org/wiki/LOLCODE)

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
