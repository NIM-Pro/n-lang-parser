# Синтаксис

В данном документе составлена синтаксическая схема модуля программы в формате PEG.

```
// Модуль - множество определений и реализаций
module = declaration*

declaration =
	| import
	| export
	| simpleDeclaration

simpleDeclaration =
	| constant
	| function
	| class

import =
	keyword("import")
	items: importList
	keyword("from")
	source: stringLiteral

importList =
	| token("*")
	| token("{")
	    $items: list(importItem)
	    token("}")

importItem =
	sourceName: ident
	(
		keyword("as")
		targetName: ident
	)?

export =
	keyword("export")
	simpleDeclaration

constant =
	keyword("const")
	name: ident
	token("=")
	value: constExpression

function =
	keyword("function")
	$name: ident?
	token("(")
	$arguments: list(pairNameType)
	token(")")
	$body: functionBlock

pairNameType =
	$name: ident
	token(":")
	$type: typeName

typeName =
	$name: ident
	$generics: (
		token("<")
		<-list(typeName)
		token(">")
	)?

functionBlock =
	token("{")
	$statements: statement*
	token("}")

statement =
	| variableDefinition
	| ifStatement
	| forCycleStatement
	| forIteratorStatement
	| whileStatement
	| switchStatement
	| breakStatement
	| returnStatement
	| expression

variableDefinition =
    keyword("let")
    $name: ident
    token(":")
    $type: typeName
    token("=")
    $value: expression
    
ifStatement =
    keyword("if")
    $statement: expression
    $thenBlock: functionBlock
    $elseBlock: ifStatementElsePart?
    
ifStatementElsePart =
    keyword("else")
    $block: functionBlock
   
cycleIdentLabel =
    $name: ident
    token(":")
    
forCycleStatement =
    $ident: cycleIdentLabel?
    keyword("for")
    token("(")
    $init: expression
    token(";")
    $statement: expression
    token(";")
    $postfix: expression
    token(")")
    $block: functionBlock

forIteratorStatement = //TODO (iterators must have some implementation for starting designing it)

whileStatement =
    $ident: cycleIdentLabel?
    keyword("while")
    token("(")
    $statement: expression
    token(")")
    $block: functionBlock

switchStatement =
    keyword("switch")
    token("(")
    $item: expression
    $name: switchStatementItemName?
    token(")")
    token("{")
    $elements: switchStatementElement
    token("}")
    
switchStatementItemName =
    keyword("as")
    $name: ident
    
switchStatementElement =
    | switchStatementCase
    | functionBlock

switchStatementCase =
    keyword("case")
    $statement: expression
    token(":")
    
breakStatement =
    keyword("break")
    $label: ident?

returnStatement =
    keyword("return")
    $value: expression

stringLiteral = //Kind of token
    // string literal begins with double quote and ends with it
    // all symbols inside quotes are included into string
    // escaping symbols is processed like inside JSON strings
    // multiline literals will be designed later
    
class =
    keyword("class")
    token("{")
    $items: classItem*
    token("}")

classItem =
    $access: classItemVisiblity
    $dynamic: classItemDynamic
    $consist: classItemConsist

classItemVisiblity =
    | keyword("public")
    | keyword("private")
    // | keyword("protected") -- this is for later use with "extends"

classItemDynamic =
    $static: keyword("static")? { return !has($static) }
    // "static" - is not dynamic and it returns "false" when found it and "true" otherwise
    
classItemConsist =
    | pairNameType
    | function

expression =
    // This is simplification.
    | binaryExpression
    | unaryExpression
    | callExpression
    | atomicExpression
    | bracketExpression
    | functionBlock
    | fieldExpression

binaryExpression =
    // This is simplification.
    $left: expression
    $operation: binaryOperation
    $right: expression
    
binaryOperation =
    | token("+")
    | token("+=")
    | token("-")
    | token("-=")
    | token("*")
    | token("*=")
    | token("/")
    | token("/=")
    | token("%")
    | token("%=")
    | token("=")
    | token("==")
    | token("**")
    | token("**=")
    | token("|")
    | token("|=")
    | token("||")
    | token("||=")
    | token("^")
    | token("^=")
    | token("^^")
    | token("^^=")
    | token("&")
    | token("&=")
    | token("&&")
    | token("&&=")

unaryExpression =
    | unaryPrefixExpression
    | unaryPostfixExpression
    
unaryPrefixExpression =
    // This is simplification.
    $operation: unaryOperator
    $value: expression
    
unaryPostfixExpression =
    // This is simplification.
    $value: expression
    $operation: unaryOperator
    
unaryOperator =
    | token("-")
    | token("!")
    | token("*")
    | token("&")

callExpression =
    $value: expression
    token("(")
    $arguments: list(expression)
    token(")")

atomicExpression =
    | stringLiteral
    | numberLiteral
    | ident
    
bracketExpression =
    token("(")
    $value: expression
    token(")")
    
constExpression = expression
    // This must be an expression that can be calculated at compile time.
    // So, this must use pure functions and constant literals only.
    // So, constants can only be a literals while purify of functions is not implemented.

fieldExpression = // TODO

list(elem, delim = token(",")) =
	$f: elem
	$o: (<-delim elem)*
	delim?
	{return $f + $o}

keyword(value) =
	$_value: ident {$_value == value}

ident = //Kind of token
	$f: letter
	$o: (letter | digit)*
	{return $f + $o}

token(value) = //Kind of token
	value
```
