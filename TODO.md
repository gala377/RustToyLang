# FTL language todo list

Things to be done as well as progress of them.

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

## Source module

* Implement file source with the utility `utf8::Reader` struct.

## Pass module

### New passes

* Pass to transform infix calls to normal calls.
* _Pass which infers function types._ - WIP (ftd - branch).
* Expr type deduction pass that fills types for the epty func decl.
* Function decls verifier. (if decl is consistant with definition)

### Interpretation or compilation

Needs all of the above to be done before starting on this one.

* Interpreter
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
