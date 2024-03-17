# wramp-transpiler
"Transpiles" named-register-wramp files (.sname) to wramp source code files (.s)

<h3>I'm not using windows, how do I use it?</h3>
Build the cargo package to your platform idk I'm still new to rust.

<h3>How do I use this?</h3>
Move the unofficial_wramp_transpiler.exe into your folder where you want to use it (or copy the file path if you don't want to move the file)
from the command line: .\unofficial_wramp_transpiler inputSNameFile

<h3>What is a named-register-wramp file?</h3>
Its a wramp source file (.s) file but instead of using $X for registers you can name them so that its easier to keep track of.
They have the .sname file extenion.

<h4>example.sname</h4>

```
# this program runs a loop which adds to total
.text
register counter 4
register total 5
register condition 6

main:
    #initialize loop vars
    addi counter, $0, 0
    addi total, $0, 0 
loop:
    #check condition
    slti condition, counter, 10
    beqz condition, loop_end

    addi counter, counter, 1
    add total, total, counter
loop_end:
    #do stuff here with total
```
<h4>Would be transpiled to this example.s</h4>

```
# this program runs a loop which adds to $5
.text

main:
    #initialize loop vars
    addi $4, $0, 0
    addi $5, $0, 0 
loop:
    #check $6
    slt $6, $4, 10
    beqz $6, loop_end

    addi $4, $4, 1
    add $5, $5, $4
loop_end:
    #do stuff here with $5
```
<h4>Todo list</h4>
- [ ] ignore comments when replacing uses of register names
- [ ] make the exe file have a shorter name
- [ ] add default names for registers like $0, $sp, etc
- [ ] remove double-empty lines after removing register declaration lines
- [ ] solve world hunger
