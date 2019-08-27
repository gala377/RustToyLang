# FTL language todo list

Things to be done as well as progress on them.

## ~~Parser module~~

* ~~Shouldn't infix calls be expressions as well? Normal calls are...~~ - REJECTED

### ~~Refactoring~~

* ~~Refactor `match ... Err(err) => Err(err)` to `let smth = self.func()?`;~~
* ~~Refactor long parse matches to `try_parse_*` functions~~
* ~~Refactor function and infix parsing to smaller functions~~
* ~~Allow function types in `parse_type`~~
* ~~change `end of file reached` fatals to `eof_fatal` call~~
* ~~refactor some matches to `unwrap_or` or `unwrap_or_else`~~
* ~~make method `save_current_ptr` and `retrieve_last_ptr`, `ref_last_ptr` for source pointer context~~.
* ~~make `save_ptr` and `retrieve_ptr` methods for parser.~~

## ~~Combinators~~

## ~~Source module~~

* ~~Implement file source with the utility `utf8::Reader` struct.~~

## Pass module

### New passes

* Pass to transform infix calls to normal calls.
* _Pass which infers function types._ - WIP (ftd - branch).
* _Expr type deduction pass that fills types for the epty func decl._ - WIP (ftd - branch)
* Function decls verifier. (if decl is consistant with definition)

## Cli application

* arguments parsing
* debug modes
* compiler phases

## `LLIR` - lower level intermediate representation.

Three address code intermediate representation for interpreter or the compiler.

### Sketch

(_Representation in further examples are just imaginary. 
LLIR doesn't have to be readable at all_)

Function definition and function declarations can stay the same.
Except the functions should have all the types filled.
(If generics shall be later implemented then generics have no 
need to be unwinded here yet). 

Yet code shall be represented in more of an imperative fashion.
As well as basic blocks shall be used for the control flow instructions.

Example

```ftl
def foo a b c [attr1 attr2]: a + b + c
```

becomes:

```llir
def foo(a:int, b:int, c:int)[attr1, atrr2] int:
    entry:
        temp_0:int = call + a:int b:int :int
        temp_1:int = call + temp_0:int c:int :int
        ret temp_1:int :int
```


Expressions need to be done in, one at a time fashion, with their 
results being stored in the temporary variables.

All variable or value usages should have fully qualified
type syntax (statement or expression followed by `:` character and
immediatly its type).
 So all expressions are typed. 
 Where `call` instruction type is called functions return type.
Instructions not returnig any value (like `jmp`) shall have
their type set to `void`.


Operators should be transformed to the function calls.
As well as infix function calls.

Example:

```ftl
def foo a b: a + b * 2 + 2
```

becomes

```llir
def foo(a:int, b:int)[] int:
    entry:
        temp_0:int = call * b:int 2:int :int
        temp_1:int = call + a:int temp_0:int :int
        temp_2:int = call + temp_1:int 2:int :int
        ret temp_2:int :int
```

`ret` and `jmp` instructions are provided for return statements and 
the control flow instructions.

Example:

```llir
def foo(a bool)[] void:
    entry:
        jmpfalse a if_false :void
    if_true:
        ...
        jmp if_ctrl :void
    if_false:
        ...
    if_ctrl:
        ...
```

Language items functions are replaced with corresponding languge item.

Example:

```ftl
decl add int int [lang_add]: int

def foo a b: @add a b
```

becomes:

```llir
def foo(a int, b int)[] int:
    entry:
        temp_0:int = a:int + b:int :int
        ret temp_0:int :int
```

Generics shall be kept in the function definition
(or declaration) and unwinded by the compiler or 
based on the calls.

Example:

```ftl
def foo a b: a + b
```

becomes:

```llir
def foo<T0, T1, T3>(a:T0, b:T1)[]: T3
    entry:
        temp_0:T3 = call + a:T0 b:T1 :T3
        ret temp_0:T3 :T3
```

## Generics

All, non filled in types, shall be considered generics.
The same way, if function type cannot be infered by simple 
expression deduction its return type shall be generic as well.

If some generic needs to be set explicitly (for example in the declaration)
then it can bo done so with the syntax:

```
decl foo<A B C> A B int: C
```

no generic bounds are to be implemented for now.

However by allowing simple generics function specialization shall be
allowed:

```
# fully generic definition
decl foo<A B C D> A B C: D

def foo a b c: a + b + c
```
***TODO*** Discuss specialiazation function syntax.


### Interpretation or compilation

Needs all of the above to be done before starting on this one.

* Interpreter
  * REPL - this needs new ast node `TopLevelExpr`.
* Compiler

## Document all public API

Including modules.

Remember that modules are documented with `//!`
while other items are documented with `///`

What's more remember about sections like `#Panics` `#Errors` and `#Examples`
and code snippets.

Documentation progress:

* `libftl_utility`
  * `utf8` module
* `libftl_source`
  * `file` module
* `libftl_session`
* `libftl_pass`
* `libftl_parser`
* `libftl_lexer`
* `libftl_error`
* `libftl_cli`
