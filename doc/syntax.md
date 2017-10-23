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
	| token("{") list(importItem) token("}")

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
	$name: ident
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

variableDefinition = //TODO
ifStatement = //TODO
forCycleStatement = //TODO
forIteratorStatement = //TODO
whileStatement = //TODO
switchStatement = //TODO
breakStatement = //TODO
returnStatement = //TODO
class = //TODO
stringLiteral = //TODO
constExpression = //TODO

list(elem, delim = token(",")) =
	$f: elem
	$o: (<-elem delim)*
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
