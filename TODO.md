# FTL language todo list

Things to be done as well as progress of them.

## Document all public API

Including modules.

Remember that modules are documented with `//!`
while other items are documented with `///`

What's more remember about sections like `#Panics` `#Errors` and `#Examples`
and code snippets.

Documentation progress:

* ~~`libftl_utility`~~
* ~~`libftl_source`~~
* `libftl_session`
* `libftl_pass`
* `libftl_parser`
* `libftl_lexer`
* `libftl_error`
* `libftl_cli`

## Parser module

* Shouldn't infix calls be expressions as well? Normal calls are...

### Refactoring

* ~~Refactor `match ... Err(err) => Err(err)` to `let smth = self.func()?`;~~
* _Refactor long parse matches to `try_parse_*` functions_ - in progress
* Refactor function and infix parsing to smaller functions
* Allow function types in `parse_type`
* ~~change `end of file reached` fatals to `eof_fatal` call~~
* ~~refactor some matches to `unwrap_or` or `unwrap_or_else`~~
* make method `save_current_ptr` and `retrieve_last_ptr`, `ref_last_ptr` for source pointer context.

## Pass module

### New passes

* Pass to transform infix passes to normal calls.
* Pass which infers function types.
* Expr type deduciton pass that fills types for the epty func decl.
* Function decls verifier. (if decl is consistant with definition)
