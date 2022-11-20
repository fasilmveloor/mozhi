# Mozhi

Mozhi is an fork of [Malluscript](https://github.com/Sreyas-Sreelal/malluscript) project. The language uses Malayalam words as keywords. The language is not strictly type-safe and uses only three datatypes strings, float and integers. Currently, mozhi is on the development phase and undergoing vigorous changes. Always check the release section for pre-built binaries for the compiler.

## Installation

Either download precompiled binaries from release page or clone this repository

`git clone https://gitlab.com/fasilmveloor/mozhi.git`

## Building

The compiler can be build as follows

``` sh
cd mozhi
cargo build --release
```

Note: You require rust compiler installed on your machine in order to build the compiler

## Executing mozhi programs

mozhi can be used in compiler mode only

### Run mozhi program files

Write the code in the file with `.മൊഴി` extension and execute it as follows

``` sh
./mozhi file_name.മൊഴി
```

## Language Syntax And Grammar

Basic arithmetic operations can be done using `+`,`-`,`*`,`/`,`%` (modulo). Every expression ends with `;`. `{` and `}` introduces a new block like c like languages.

### Basic I/O operations

* Printing or writing to console can be done as follows

    ```മൊഴി
    "hello world" എഴുതുക;
    variable എഴുതുക;
    "Onnum randum kottiyal " + 1+2 + " aane" എന്നു എഴുതുക; 
    പ്രിന്റ് "നമ്പർ = ";
    ```

  * Alternate Keywords
    * `പ്രിന്റ് ചെയ്യുക`
    * `എന്നു പ്രിന്റ് ചെയ്യുക`
    * `പ്രദർശിപ്പിക്കുക`
    * `എന്നു പ്രരദർശിപ്പിക്കുക`
    * `എന്നു എഴുതുക`
    * `കാണിക്കുക`
    * `എന്നു കാണിക്കുക`

* In order to get keyboard input from the user
  
  ```മൊഴി
  variable ഇൻപുട് ചെയ്യുക;
  ```

Note: If the input contain only integers, it get automatically converted to integer, if it contain floating point, it get converted to float, all the other cases, it get converted to string in current scenario(It may be refactor in future update)

### Datatypes and Storage

mozhi is not strictly type-safe.The language, currently, supports only integers, floats and string literals as datatypes.
unlike malluscript, there is no keyword for variable decleration in mozhi . It follows python similar syntax for variable decleration.
ie, the variable get automatically declared on the time of assigning if and only if it is not previously declared

To assign it some value

```മൊഴി
variable_name = 1;
second_var = "ente string";
```

## Conditional Statements And Expressions

The conditional expression has the following syntax

```മൊഴി
i ഉം 0 ഉം തുല്യമാണെങ്കിൽ {

} അല്ലെങ്കിൽ {

}
```

The above snippet checks whether i equal to 0 and if yes the code in the first block will execute otherwise block defined by `seri_allel` will get executed.

In general

* `i ഉം 0 ഉം തുല്യമാണെങ്കിൽ` checks if i is equals to 0
* `i ഉം 0 ഉം തുല്യമല്ലെങ്കിൽ` checks if i not equals to 0
* `i നെകാൾ 0 ചെറുതാണെങ്കിൽ` checks if 0 is less than i
* `i നെകാൾ 0 വലുതാണെകിൽ` checks if 0 is greater than i
* `i നെകാൾ 0 വലുതോതുല്യമോആണെങ്കിൽ` checks if 0 is greater than or equal to i
* `i നെകാൾ 0 ചെറുതോതുല്യമോആണെങ്കിൽ` checks if 0 is less than or equal to i
* `i == 0 ആണെങ്കിൽ` checks if i is equals to 0
* `i != 0 ഉം ആണെങ്കിൽ` checks if i not equals to 0
* `i < 0 ആണെങ്കിൽ` checks if 0 is less than i
* `i > 0 ആണെങ്കിൽ` checks if 0 is greater than i
* `i >= 0 ആണെങ്കിൽ` checks if 0 is greater than or equal to i
* `i <= 0 ആണെങ്കിൽ` checks if 0 is less than or equal to i

## Iterative Statements Or Loops

The loops in mozhi has two types

### While loops

while loop in mozhi will execute until given condition is false

#### example

```മൊഴി
i ഉം 0 ഉം തുല്യമാകുന്നതുവരെ ആവർത്തിക്കുക {
  i = i-1;
}

```

The above code snippet will execute untill i and 0 become equal

In general

* `i ഉം 0 ഉം തുല്യമാകുന്നതുവരെ ആവർത്തിക്കുക` run until i is equals to 0
* `i ഉം 0 ഉം തുല്യമല്ലാതാകുന്നതുവരെ ആവർത്തിക്കുക` run until i not equals to 0
* `i നെകാൾ 0 ചെറുതാകുന്നതുവരെ ആവർത്തിക്കുക` run until 0 is less than i
* `i നെകാൾ 0 വലുതാകുന്നതുവരെ ആവർത്തിക്കുക` run until is greater than i
* `i നെകാൾ 0 വലുതോതുല്യമോആകുന്നതുവരെ ആവർത്തിക്കുക` run until if 0 is greater than or equal to i
* `i നെകാൾ 0 ചെറുതോതുല്യമോആകുന്നതുവരെ ആവർത്തിക്കുക` run until if 0 is less than or equal to i
* `i == 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until i is equals to 0
* `i != 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until if i not equals to 0
* `i < 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until 0 is less than i
* `i > 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until 0 is greater than i
* `i >= 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until 0 is greater than or equal to i
* `i <= 0 ആകുന്നതുവരെ ആവർത്തിക്കുക` run until 0 is less than or equal to i

### For loops

for loop in mozhi will execute a given number of times.

##### example

``` മൊഴി
5 തവണ ആവർത്തിക്കുക {
  i = i-1;
}

x = 10
x തവണ ആവർത്തിക്കുക {
  i = i-1;
}

```

* Alternative Keywords for തവണ
  * `പ്രാവശ്യം`
  * `വട്ടം`

## Example

A simple program to find factorial in malluscript would be

```മൊഴി
പ്രിന്റ് "നമ്പർ = ";
num ഇൻപുട് ചെയ്യുക;
factorial = 1;

0 ഉം num ഉം തുല്യമാകുന്നതുവരെ ആവർത്തിക്കുക {
    factorial = factorial * num;
    num = num -1;
}

"ഫാക്ടോറിയൽ = " + factorial + "\n" പ്രിന്റ് ചെയ്യുക;
```

More examples can be found in [examples](examples)

## Notes

The language as mentioned above is under the development phase and its structure can change overnight, suddenly. This language is not meant to disrespect anyone and wrote just for fun.

Any kind of contribution is always welcome. If you have any ideas or improvements to provide for this project open a pull request or if you have any difficulties using this language open an issue :)
