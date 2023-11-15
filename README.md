# ![Nosr Object Spec Representation](./assets/nosr.svg)

Nosr Object Spec Representation (pronounced
[/noʊˈsɝ/](http://ipa-reader.xyz/?text=no%CA%8A%CB%88s%C9%9D))

## Rationale

In order to represent structured data, such as configuration files, developers
often reach for a format such as [JSON](https://www.json.org/json-en.html).
JSON, however flexible it is, does doesn't totally solve every imaginable use
case. Some people really want extended types. Others want functions and
metaprogramming support.

**Nosr** is none of that, because it doesn't care about types. Nor does it give
you fancy functions or metaprogramming support. Instead, it gives you access to
a dead-simple tree structure that you, the programmer, are responsible for
interpreting. Don't expect anything out of **nosr** except a parse tree (and a
few utility functions to parse the types as your program sees fit).

Good luck. Have fun. May the odds be ever in your favor.

## Spec

I'll probably get around to writing some BNF or other formal grammar eventually.
Have some informal definitions and a few notional examples in the meantime.

### Encoding

[UTF-8](https://en.wikipedia.org/wiki/UTF-8) ... what else did you expect?

### Parse Tree

All data is parsed into a tree structure defined by two kinds of nodes:
*sequences* and *bindings*. All *scalar* values live at the leaves of
the parse tree.

**Sequences** are bounded by `[` and `]` characters. Sequence elements are
delimited by `,` or `;` or a new line.

**Bindings** are defined as a key/value pair, delimited by a `:` character.

> Wait ... where are the maps? dictionaries? associative arrays?!? Well, you're
> in luck! That's just a sequence of bindings!

Because parsers are obsessive compulsive perfectionists, we do have to define a
few other special tokens.

**Texts** are bounded by a pair of `"` characters. Modifies parse rules such
that the only characters with special meaning are the double-quote (`"`) and
escape (`\`) characters. Comes with new lines and other whitespace. Batteries
not included.

**Escapes** are preceded by a `\`. For instance, `\n` is a newline literal, `\"`
is a double-quote literal, and `\:` is a colon literal.

**Scalars** are any other string of characters.

## Examples

```
[ hello; world; of; token; sequences; ]
```

```
[
    letters: abcd
    numbers: 1234
    base64!: YmluYXJ5IQ==
    escape\:me : this is a quote\:\"
    "text me" : "\
        behold: something that modifies
        the parse [state machine] rules;
        so long as you escape \" chars"
]
```

```
you could also just write a plain text
file and call that a nosr file so long
as it didn't contain any reserved chars
```
