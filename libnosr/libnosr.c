/*

{
    x: "a\"b\"c",
    y: [1, 2, 3],
    z: { inner: text }
}

*/

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>

#define OK 0
#define NOT_OK 1
#define EARLY_EOF 2
#define TODO 255

typedef int retcode_t;

typedef struct
{
    //! position of the first character in the string
    const char* start;

    //! position after the last character of the string; will be '\0' for null-terminated strings
    const char* end;
} text_t;

typedef enum
{
    BACKSLASH,
    BRACKET_LEFT,
    BRACKET_RIGHT,
    COLON,
    COMMA,
    NEWLINE,
    QUOTE,
    SEMICOLON,
    SYMBOL
} tok_type_t;

typedef struct
{
    text_t lexeme;
    tok_type_t type;
} tok_t;

const char *lex_advance(const char* s)
{
    return s + sizeof(char);
}

retcode_t lex_next_quote(const text_t* source, tok_t* tok)
{
    return TODO;
}

retcode_t lex_next_symbol(const text_t* source, tok_t* tok)
{
    return TODO;
}

retcode_t lex_next(const text_t* source, tok_t* tok)
{
    text_t lexeme = {.start = source->start, .end = lex_advance(source->start) };
    while (lexeme.start < lexeme.end)
    {
        char ch = *lexeme.start;
        switch (ch)
        {
        case '\\':
            *tok = (tok_t){.type = BACKSLASH, .lexeme = lexeme};
            return OK;
        case '[':
            *tok = (tok_t){.type = BRACKET_LEFT, .lexeme = lexeme};
            return OK;
        case ']':
            *tok = (tok_t){.type = BRACKET_RIGHT, .lexeme = lexeme};
            return OK;
        case ':':
            *tok = (tok_t){.type = COLON, .lexeme = lexeme};
            return OK;
        case ',':
            *tok = (tok_t){.type = COMMA, .lexeme = lexeme};
            return OK;
        case '"':
            return lex_next_quote(&(text_t) { .start = lex_advance(lexeme.start), .end = source->end }, tok);
        case '\n':
            *tok = (tok_t){.type = NEWLINE, .lexeme = lexeme};
            return OK;
        case ';':
            *tok = (tok_t){.type = SEMICOLON, .lexeme = lexeme};
            return OK;
        case '\r':
        case '\t':
        case ' ':
            lexeme.start = lex_advance(lexeme.start);
            lexeme.end = lex_advance(lexeme.end);
            break;
        default:
            return lex_next_symbol(&(text_t) { .start = lexeme.start, .end = source->end }, tok);
        }
    }

    return EARLY_EOF;
}

retcode_t lex_print(const text_t* source, const tok_t* tok)
{
    const size_t offset_from = tok->lexeme.start - source->start;
    const size_t offset_to = tok->lexeme.start - source->end;

    // print type
    switch (tok->type)
    {
    case BACKSLASH: // '\\'
        printf("(BACKSLASH @ %zu)", offset_from);
        break;
    case BRACKET_LEFT: // '['
        printf("(BRACKET_LEFT @ %zu)", offset_from);
        break;
    case BRACKET_RIGHT: // ']'
        printf("(BRACKET_RIGHT @ %zu)", offset_from);
        break;
    case COLON: // ':'
        printf("(COLON @ %zu)", offset_from);
        break;
    case COMMA: // ','
        printf("(COMMA @ %zu)", offset_from);
        break;
    case NEWLINE: // '\n'
        printf("(NEWLINE @ %zu)", offset_from);
        break;
    case SEMICOLON: // ';'
        printf("(SEMICOLON @ %zu)", offset_from);
        break;
    default:
        return TODO;
    }

    return OK;
}

retcode_t main(int argc, char const* argv[])
{
    int is_ok = OK;
    char* source_str = "\\[]:,\n \r\t;";
    size_t source_len = strlen(source_str);
    text_t source = { .start = &source_str[0], .end = &source_str[source_len] };
    text_t remain = source;
    puts("tokens...");

    for (int i = 0; i < source_len; i++)
    {
        remain.start = &source_str[i];
        remain.end = &source_str[source_len];
        tok_t token;
        memset(&token, 0, sizeof(token));
        if (OK != lex_next(&remain, &token)) {
            puts("bad stuff happened");
            is_ok = NOT_OK;
            break;
        }
        lex_print(&source, &token);
        puts("");
    }

    return is_ok;
}
