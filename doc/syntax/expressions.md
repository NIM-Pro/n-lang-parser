[знак]: tokens.md#Знак
[идентификатор]: tokens.md#Идентификатор
[выражение]: #Выражения

# Выражения
Здесь описаны возможные выражения, используемые для манипуляции над данными. Учтите, что в статье описаны упрощённые рекурсивные синтаксические структуры и при реализации данных структур стоит преобразовывать грамматику в более производительный, либо в более простой вид.

Выражение может быть:
* [Бинарным](#Бинарные-выражения)
* [Унарным](#Унарные-выражения)
* [Выражением вызова](#Выражения-вызова)
* [Выражением группировки](#Выражения-группировки)
* [Блоком высказываний](#Блок-высказываний)
* [Структурным](#Структурное-выражение)
* [Атомарным](#Атомарное-выражение)

## Бинарные выражения
Бинарные (двоичные) выражения состоят из двух операндов и [знака][знак] оператора. Типичным примером двоичного выражение является сложение двух чисел `2 + 3`, где числовой литерал `2` является левым операндом, `3` - правым операндом, а `+` - [знаком][знак] операции.

На данный момент предусмотрены следующие [знаки][знак] двоичных операций:
* `+` - сложение
* `+=` - добавление к левому
* `-` - вычитание
* `-=` - вычитание из левого
* `*` - умножение
* `*=` - умножение с записью в левый
* `/` - деление
* `/=` - деление с записью в левый
* `%` - остаток от деления
* `%=` - остаток от деления с записью
* `=` - присваивание левому значение правого
* `**` - возведение левого в степень правого
* `**=` - возведение левого в степень правого с записью
* `|` - побитовое `или`
* `|=` - побитовое `или` с записью
* `||` - логическое `или`
* `||=` - логическое `или` с записью
* `^` - побитовое `исключающее или`
* `^=` - побитовое `исключающее или` с записью
* `^^` - логическое `исключающее или`
* `^^=` - логическое `исключающее или` с записью
* `&` - побитовое `и`
* `&=` - побитовое `и` с записью
* `&&` - логическое `и`
* `&&=` - логическое `и` с записью
* `==` - отношение эквиваленции
* `!=` - отношение неэквиваленции
* `<` - отношение `левое меньше правого`
* `<=` - отношение `левое меньше или равно правому`
* `>` - отношение `левое больше правого`
* `>=` - отношение `левое больше или равно правому`

## Унарные выражения
Унарные выражения, в отличии от бинарных, состоят из одного операнда и [знака][знак] операции. Однако, это свойство привносит новые нюансы: [знак][знак] унарного выражения может находиться как перед операндом, так и после него.

Поэтому, унарные операции делятся на:
* Префиксные унарные операции (`знак операнд`).
Типичным примером префиксной операции является `отрицание знака числа` (`-3`, `-10`, и т.д.).
* Постфиксные унарные операции (`операнд знак`).
Примером постфиксной унарной операции является операция `факторилизации` (`2!`, `5!`, и т.д.).

Для префиксных операций предусмотрены следующие [знаки][знак]:
* `-` - отрицание знака числа
* `!` - логическое `не`
* `*` - раскрытие ссылки
* `&` - получение ссылки
* `++` - преинкремент
* `--` - предекремент

Для постфиксных операций предусмотрены следующие [знаки][знак]:
* `++` - постинкремент
* `--` - постдекремент

## Выражения вызова
Вызов функции (либо вызова функции-обработчика оператора вызова для значения заданного типа) производится следующим образом:
* Вычисляются в неопределённом порядке:
    * Значение, к которому применяется вызов;
    * Значение каждого из передаваемых аргументов;
* Происходит передача аргументов и управления в вызываемую функцию.

Синтаксически вызов определяется как выражение, за которым следует [знак][знак] `(`, за которым следует список [выражений][выражение], которые называются `аргументами вызова функции`, которые перечисляются через знак `,`. Завершает конструкцию [знак][знак] `)`.
Примеры вызова функции: `my_func(5, -1)`, `my_obj.my_field()`, `my_obj.my_method()`, `func_which_returns_func()()`.

Вызов функции является выражением потому, что синтаксически приравнивается к значению того же типа, что является возвращаемым функцией.

Помимо вызова функции, под определение `выражения вызова` попадает так же `выражение обращения по индексу`, т.е. то же самое, но при этом используются [знаки][знак] `[` и `]` в качестве скобок. За примерами ходить далеко не нужно, ведь все мы знаем, что такое `получение элемента массива`: `my_array[i]`.

## Выражения группировки
Группировка выражений осуществляется при помощи [знаков][знак] `(` и `)`. Это похоже на математические скобки, ведь группировка используется для явного указания компилятору последовательности вычисления выражения, игнорируя заданные в компиляторе приоритеты операций. Чаще это используется в бинарных выражениях, например `(2 + 2) * 2`.

## Блок высказываний
Блок высказываний является подобией блока тела функции, за тем лишь исключением, что, в отличии от тела функции, где ожидается наличие оператора `return`, результирующими выражениями в нём считаются последние в каждой из веток исполнения.

## Структурное выражение
Структурным выражением называется обращение к части составного типа, такой как *метод* или *поле*.

Типичным примером структурного выражения, будет, как раз, обращение к полю объекта: `my_obj.my_field`.

Что важно, слева от [знака][знак] `.` должно стоять выражение, а справа - идентификатор. Получение данных по строковому имени поля возможно только средствами `рефлексии`.

## Атомарное выражение
Атомарным выражением являются литералы и [идентификаторы][идентификатор]. Чтобы вам было понятнее что это, напомню, что литералы делятся на:
* Строковые
* Числовые
* Литералы списка