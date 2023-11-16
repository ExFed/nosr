# ![Nosr Object Spec Representation](./assets/nosr.svg)

*Nosr Object Spec Representation*

Pronounced [/noʊˈsɝ/](http://ipa-reader.xyz/?text=no%CA%8A%CB%88s%C9%9D), or
[/noʊ sinˈjɔɹ/](http://ipa-reader.xyz/?text=no%CA%8A%20sin%CB%88j%C9%94%C9%B9)
... or [/ˈnɒzər/](http://ipa-reader.xyz/?text=%CB%88n%C9%92z%C9%99r). Whatever
floats your boat.

## Rationale

In order to represent structured data, such as configuration files, developers
often reach for formats such as [JSON](https://www.json.org/json-en.html). JSON,
however flexible it is, doesn't complete solve every imaginable use case. Some
people really want extended types. Others want functions and metaprogramming
support. Clearly JSON just isn't good enough.

Well, if you're one of those people, **nosr** probably won't help you, because
has fewer types than JSON, and it definitely doesn't give you any fancy
functions or metaprogramming support. But it gives you access to a dead-simple
tree structure that you, the programmer, get to interpret all on your own. Don't
expect any special formatting out of **nosr** except a tree.

But wait! There's more! The "parsing" bits are actually encoded into the API!
Sometimes you really absolutely need a 64-bit unsigned integer instead of a
goofy 53-bit signed integer (apparently some JSON parsers just use `double` to
represent numbers). Or maybe you need a rational, because `1/3` is much more
precise than `0.333`. Well, you *could* add more types to the format ... or you
could just admit that parsing things into pre-baked categories is either really
silly, or requires [way more math](https://en.wikipedia.org/wiki/Dependent_type)
than most people are willing to stomach. Hence: **nosr**.

Good luck. Have fun. May the odds be ever in your favor.

## Spec

I'll probably get around to writing some BNF or other formal grammar eventually.
Have some informal definitions and a few notional examples in the meantime.

**Buyer beware!** This whole idea is completely hair-brained and likely to
entirely not work, so I reserve the right to arbitrarily and capriciously modify
the specification in its entirety at any time ... including this disclaimer.

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

**Whitespace** is anything that falls under the "normal" definition. Spaces,
tabs, newlines, carriage returns.

**Texts** are bounded by a pair of `"` characters. Modifies parse rules such
that the only characters with special meaning are the double-quote (`"`) and
escape (`\`) characters. Comes with new lines and other whitespace. Batteries
not included.

**Escape sequences** are preceded by a `\` character. For instance, `\n` is a
newline literal, `\"` is a double-quote literal, and `\:` is a colon literal.

**Scalars** are any other string of characters which are bounded by
non-whitespace characters.

Let's also define syntax for comments, because unlike
[Douglas
Crockford](https://web.archive.org/web/20190112173904/https://plus.google.com/118095276221607585885/posts/RK8qyGVaGSr),
I'm not a monster.

**Line Comments** are normal C-style line comments: `//` until the next newline.
Discarded by the parser.

**Block Comments** are normal C-style block comments, bounded by `/*` and `*/`
tokens. Discarded by the parser.

### The API

As mentioned above, the point of **nosr** isn't to jam a bunch of sophisticated
math into a data format. No, that's been done. Don't get me wrong, languages
like [Dhall](https://dhall-lang.org/) and [CUE](https://cuelang.org/) are wicked
awesome and deserve attention, but that's not the goal of **nosr**.

Instead, the serialized format simply encodes a tree structure. The rest of the
heavy lifting comes from the API, which defines "types" in terms of function
calls. In the event you only need a few values within a document, this should
reduce the time spent parsing, as it means your program will only parse the
sections that it absolutely needs, and no more.

Of course, you could just argue **nosr** is just a lexical analyzer with a silly
name. You'd be mostly right.

Alright, without further ado, let's define a few basic operations with some very
rough pseudocode:

* `document(filename: string): nosr_node`

  Parses the root node of a document.

* `text(node: nosr_node): string`

  Parses a node as a string literal.

* `uint64(node: nosr_node): uint64`

## Examples

### Texts

A file can just be a single scalar:

    "hello world!"

The parser should also support cases like this:

    you could also just write a plain-text
    file and call it \"nosr\" so long as it
    appropriately escapes reserved chars

### Comments

    /*
     * The tyranny of comments.
     */
    "Ceci n'est pas une pipe." // This, however, is a comment.

### "Numbers"

Keep calm, use the **nosr** API, and carry on.

    12.34 // what did you expect? convenience?

### Vectors

    [ some, kind, of, "vector" ]

### Tables

    {
        letters: abcd
        numbers: 1234
        base64!: YmluYXJ5IQ==
        escape\:me : have a double quote\:\"
        "text me":
            "
            behold: a block of text modifies
            the parse [state machine] rules;
            so long as you employ escape
            sequences, like \" and \\, your
            data will be okay!
            "
    }
