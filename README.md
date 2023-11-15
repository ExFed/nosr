# ![Nosr Object Spec Representation](./assets/nosr.svg)

Nosr Object Spec Representation (pronounced
[/noʊˈsɝ/](http://ipa-reader.xyz/?text=no%CA%8A%CB%88s%C9%9D))

## Rationale

In order to represent structured data, such as configuration files, developers
often reach for a format such as [JSON](https://www.json.org/json-en.html).
JSON, however flexible it is, does doesn't totally solve every imaginable use
case. Some people really want extended types. Others want functions and
metaprogramming support.

Well **nosr** probably won't help you, because it doesn't care about types, and
it definitely doesn't give you any fancy functions or metaprogramming support.
Instead, it gives you access to a dead-simple tree structure that you, the
programmer, are responsible for interpreting. Don't expect anything out of
**nosr** except a tree (and a few utility functions to parse scalars as your
program sees fit).

Good luck. Have fun. May the odds be ever in your favor.

## Spec

I'll probably get around to writing some BNF or other formal grammar eventually.
Have some informal definitions and a few notional examples in the meantime.

**Buyer beware!** I reserve the right to arbitrarily and capriciously modify the
specification in its entirety at any time ... including this disclaimer.

### Encoding

[UTF-8](https://en.wikipedia.org/wiki/UTF-8) ... what else did you expect?

### Parse Tree

All data is parsed into a structure defined by two kinds of trees: ***tables***
and ***vectors***. All ***scalar*** values live at the leaves of the parse tree.

**Tables** are bounded by `{` and `}` characters. Contain a sequence of pairs,
which are defined as a pair of values separated by a `:` character and delimited
by a `,` or `;` or newline character. Tolerates a trailing delimiter. It's
probably smart to say something like "keys are always strings", but I'm lazy, so
we'll burn that bridge when we come to it.

**Vectors** are bounded by `[` and `]` characters. Sequence elements are
delimited by a `,` or `;` or newline character. Tolerates a trailing delimiter.
Essentially shorthand for a table with sequential unsigned integers for keys.

Parsers are obsessive compulsive perfectionists, so we have to define a little
more syntax.

**Texts** are bounded by a pair of `"` characters. Modifies parse rules such
that the only characters with special meaning are the double-quote (`"`) and
escape (`\`) characters. Comes with new lines and other whitespace. Batteries
not included.

**Escape sequences** are preceded by a `\`. For instance, `\n` is a newline
literal, `\"` is a double-quote literal, and `\:` is a colon literal.

**Scalars** are any other string of characters which are bounded by
non-whitespace characters.

Let's also define syntax for comments, because unlike
[Douglas
Crockford](https://web.archive.org/web/20190112173904/https://plus.google.com/118095276221607585885/posts/RK8qyGVaGSr),
I'm not a monster.

**Line Comments** are normal C-style line comments: `//` until the next newline.

**Block Comments** are normal C-style block comments, bounded by `/*` and `*/`
tokens.

## Examples

### Texts

    "hello world!"

    you could also just write a plain-text
    file and call it \"nosr\" so long as it
    appropriately escapes reserved chars

### Comments

    /*
     * The tyranny of comments.
     */
    "Ceci n'est pas une pipe." // This, however, is a comment.

### "Numbers"

    "12.34" // what did you expect? convenience?

### Vectors

    [ some, kind, of, "vector"; ]

### Tables

    {   letters: abcd
    ;   numbers: 1234
    ;   base64!: YmluYXJ5IQ==
    ;   escape\:me : this is a quote\:\"
    ;   "text me" :
            "
            behold: something that modifies
            the parse [state machine] rules;
            so long as you escape \" chars
            "
    }
