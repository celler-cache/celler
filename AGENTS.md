# Instructions for AI Agents

This file applies to all AI coding agents working in this repository.

## Do Not Write Code

You are allowed to explore the code base to explain how it works, or to find bugs
or security issues.

Do not write new code. This includes new features and new tests.

Do not change the database layout. Remind the user about proper migrations for
any database changes. See server/src/database/migration for migrations.

You are allowed to fix compilation issues or make other trivial fixes. These
include local changes, such as changing a loop to an iterator, that can be
reasoned about without having knowledge of the larger code base.

You are allowed and encouraged to fix security issues, if the fix can be
reasoned about locally. For security fixes that require larger changes, explain
the problem to the user and let them fix it.

## Write Beautiful Documentation

You are allowed to write documentation in the book, if it is high quality and
follows documentation best practices. Do not mix user documentation with
developer documentation in the book. The best way to avoid mixing them is to
keep them in separate sections.

User-facing documentation in particular must be clear and use simple language.
It must be understandable by people comfortable with managing a NixOS system.

Developer documentation in the book must not duplicate code comments. Developer
documentation in the book should focus on design decisions and high-level
overview. Refer from the code to the book for these topics by mentioning which
section or chapter in the book contains relevant information.

You are allowed to write and improve code comments, but keep them short! Focus
on information that is not obvious in the code, such as the intention or how it
relates to the larger design. Do not write comments that just restate the
mechanics of what the code does, except when the code is too complex. Suggest
simplifications to the code that make the intention clearer, if possible, to
avoid having to explain it.

In general, do not duplicate information across the code base. Document one
topic in one place and refer to it from other places. There are exceptions when
explanations need to be in two places, such as in the CLI help output and the
book.

## Mark AI Contributions

Mark any changes you make with a `Co-Authored-By:` in commit messages in a way
that allows GitHub to recognize the model.
