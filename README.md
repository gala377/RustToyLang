# FTL language

`FTL` stands for `FunToyLanguage`. Yes you read that right.
Its fully qualified name is "FunToyLanguage language".

It is a simple functional language inspired with `Haskell`,
yet if you want to use something *good* just use `Haskell` .

## Syntax

In the later syntax definitions there will be some variables used.
Namely `ident`, `op`, `int_lit`. Writing `EBNF` notation for them
is still _work in progress_.

One note about the operators is that the parenthesis characters, namely
`(`, `)`, `[` and `]` cannot be part of any operator. But `{` and `}` can
so use those.

### Comments

Only line comments are present.
They start with the `#` character and are in effect to the end of the line.

#### Examples

```ftl
# this is some comment

def test: 5 # some comment after function definition

# No multiline comments
# but no harm in doing it
# this way
```

### Function definition

```ebnf
func_def = "def", ident, {ident}, ["[", {ident}, "]"], ":", expr
```

Function definition stars with the keyword `def`.
Following is the identifier being the function name.
Next are function arguments. Each being one identifier,
separated by spaces. There can be 0 or more arguments.
After which, optionally, there is a function attribute list,
0 or more identifiers separated by spaces and surrounded with
parenthesis.
Following is the colon and then expression being the functions body.

#### FuncDef Examples

```ftl
def multiple_args a b c: a + b + c
def return_some_val: 2+2*2
def with_attributes [attr1 attr2]: 2
def args_and_attrs a b c [at_1 at_2 at_3]: 3
```

### Function declaration

```ebnf
func_decl = "decl", ident, {ident}, ["[", {ident}, "]"], ":", ident
```

Function declarations is similar to the function definition.
Differences being that instead if the `def` keyword `decl` keyword
is used, function arguments become its types and function body
as well become return type of the function (being `void` if function
doesn't return anything).

#### FuncDecl Examples

```ftl
decl nop [lang_nop] : void
decl add int int [lang_add inline] : int
decl foo int int: int
decl test3 [test1 test2] : int
```

### Infix declaration

Infix is a special function that can only be called
as a binary operator and its name is not an identifier
but an operator. What's more as infixes are treated
as the binary operators each one of them has its
own precedence.

```ebnf
infix_def = "infix", int_lit, op, {ident}, ["[", {ident}, "] "], ":", expr
```

Infix definition starts with the `infix` keyword after which
there is an interger literal being its precedence.
Then there is the name of the infix being an operator.
After that rest of the difinition stays like in the
normal function case.

Infixes cannot have declarations.

#### InfixDef Exmaples

```ftl
infix 5 @@ a b: a + b
infix 10 $ func expr: @func expr
infix 50 - a b: @sub a b
infix 5 <==> a b [inline debug] : a * b
```

### Expressions

* Literals
  * Integer. Example: `5`,
* Function call. Starts with `@` being followed by the function name and its arguments.
Has the highest precedence. Exmaple: `@add 1 2`
* Infix function call. Starts with ``` being followed by the function name. Takes
the left expression as the first argument and the right one as the second. Example: ``1 `add 2``.
* Binary operator call. Calls operator with the left expression as its first argument
and its right one as the second argument. Example `1 + 2`.
* Parenthesed expression. Expression surrounded by the parenthesis. Example: `(2+2)*2`
* Identifiers. For now used only to pass functions as arguments.
Example: `@call some_func`

### Types

For now only 2 basic types are supported:

* `int`
* `void`

Function types look like so:

```ebnf
func_type = "(", {type}, ")", type
```

#### Function Type Example

```ftl
decl test (int)int (int)int: (int int) int
```

## Example code

Mostly used for testing, but there is no reason as to why
I shouldn't let it be here.

```ftl
decl nop [lang_nop] : void

decl add int int [lang_add inline] : int
decl mult int int [lang_mult] : int

infix 5 @@ a b: a + b
infix 10 $ func expr: @func expr
infix 50 - a b: @sub a b
infix 50 + a b: @add a b
infix 100 * a b: @mult a b

def multiple a b c: a + b + c
def call_mult: @multiple 1 2 3 + 2

decl foo int int : int
def foo a b: a + b

def bar: 1 - 2 + 3 `foo_bar 4 $ 5 * 0

def foo_bar: @bar @@ 1 + 2 + @foo 3 (2+2*2) $ 2

def test: 2 + 2 * 2
infix 5 <==> a b : 1 `foo 2 `foo 3 `foo 4 + (
def test3 [test4 test1] : 2+2*2*2*2*2*2

decl test3 [test1 test2] : int
```
