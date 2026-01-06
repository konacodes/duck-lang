" Vim syntax file for Duck programming language
" Language: Duck
" Maintainer: konacodes

if exists("b:current_syntax")
  finish
endif

" Comments
syn match duckComment "--.*$" contains=duckTodo
syn keyword duckTodo contained TODO FIXME XXX NOTE HACK

" Strings with interpolation
syn region duckString start=/"/ skip=/\\"/ end=/"/ contains=duckInterpolation,duckEscape
syn region duckInterpolation start=/{/ end=/}/ contained contains=duckIdentifier,duckNumber,duckOperator
syn match duckEscape /\\./ contained

" Numbers
syn match duckNumber "\<\d\+\>"
syn match duckNumber "\<\d\+\.\d\+\>"

" The essential quack keyword (special highlighting)
syn keyword duckQuack quack

" Control flow keywords
syn keyword duckConditional if then otherwise
syn keyword duckRepeat while do repeat times for each in until
syn keyword duckStatement return break continue

" Declaration keywords
syn keyword duckKeyword let be becomes define taking as struct with

" Boolean and null
syn keyword duckBoolean true false
syn keyword duckNull null

" Operators
syn keyword duckOperator and or not at length is push pop
syn match duckOperator "=="
syn match duckOperator "!="
syn match duckOperator "<="
syn match duckOperator ">="
syn match duckOperator "<"
syn match duckOperator ">"
syn match duckOperator "+"
syn match duckOperator "-"
syn match duckOperator "\*"
syn match duckOperator "/"
syn match duckOperator "%"
syn match duckOperator "->"

" Built-in functions
syn keyword duckBuiltin print input random floor ceil abs sqrt pow min max
syn keyword duckBuiltin type-of len string number range list

" Brackets (blocks)
syn match duckBracket "[\[\]]"

" Identifiers (including hyphenated ones)
syn match duckIdentifier "\<[a-zA-Z_][a-zA-Z0-9_-]*\>"

" Highlighting links
hi def link duckComment Comment
hi def link duckTodo Todo
hi def link duckString String
hi def link duckInterpolation Special
hi def link duckEscape SpecialChar
hi def link duckNumber Number
hi def link duckQuack Keyword
hi def link duckConditional Conditional
hi def link duckRepeat Repeat
hi def link duckStatement Statement
hi def link duckKeyword Keyword
hi def link duckBoolean Boolean
hi def link duckNull Constant
hi def link duckOperator Operator
hi def link duckBuiltin Function
hi def link duckBracket Delimiter

let b:current_syntax = "duck"
