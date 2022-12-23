# AoCreate
Little helper to have an easy Advent of Code with rust and focus on coding ;D

------------


### How to install it
1. Download the release .exe at the root directory
2. Add the .exe to the path
3. Use it! And have fun solving the beloved riddles at [Advent of Code](https://adventofcode.com "Advent of Code")

### How to use it
Two simple commands:
1. `aocreate year [year] [name]`
	* Creates a folder (`[name]`) in the current directory: this will be the project directory
    * If you don´t specify the `[name]` it will become the `[year]` or more respectively `AoC-[year]`
	* If you don´t specify the `[year]` the current year will be the year of the project and therefore the project directory
2. `aocreate day [day]`
	* Creates a day folder (written-out `day`: `[wday]`) inside the project/src directory with the input data and the example data of the day. In addition a mod.rs file will be created with three function: `[wday]_test` (run with example data), `[wday]_run` (run with the input data) and `[wday]` (has the data as a string as a argument)
	* If day is not specified it will run as the current day

------------


That´s it!
Have fun!