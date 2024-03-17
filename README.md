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

<h4>Example .sname file:</h4>

```
#this program runs a loop which adds to a total
.text
register counter 4
register total 5
register condition 6
register iterations 10

main:
    #initialize loop vars
    addi counter, $0, 0
    addi total, $0, 0 
loop:
    #check condition
    slt condition, counter, iterations
    beqz condition, loop_end

    addi counter, counter, 1
    add total, total, counter
loop_end:
    #do stuff here with total
```
