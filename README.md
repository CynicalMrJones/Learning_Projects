# ***Learning Projects***

This is an always updating list of projects that I have learned from or are just plain cool. Most of the programs that are here aren't going to be the most efficient but they taught me something interesting. I will do my best to explain what I learned and how it is an improvement where applicable. My programming background is in java (for school), Lua and about 36 seconds of JavaScript.

### **Dice Simulator**
I love DnD so I figured making a dice roller would be a fun way to practice programming. Originally, I was just going to have the program roll the standard DnD dice (d2, d4, d6, d8, d10, d20). The idea expanded when my girlfriend mentioned it would be more fun if you could roll any type of dice, even ones that aren't possible. With this in mind I decided to write this program in rust. Rust is a very new and different language for me with a lot of rough learning curves.

1) Version 1
The first iteration of this was very poorly written but I did learn a good amount. Most of the loops in it are while loops where you make declare a variable, "i" in this case, an use it as your iterator. I am now aware that rust has baked in iterator functions but I was just programming with what I knew. One of the major problems this project has was making individual functions for each and every dice. A very good programming rule is that "If you are going to reuse code at least thrice then it should probably be a function" -Albert Einstein (probably). Another downfall this program had was its inability to use dice that didn't exist, examples being a d3 or d678. Also the looper() function have such a large if statement with lots of recurring code is not my proudest moment. Overall this was a good first attempt and it does give an output for all of the valid dice options.

2) Version 2
This was a straight up improvement compared to the old version. The old one was a 198 line mutant only implmenting very basic techniques. The new version was a 66 line freedom fueled masterpiece. The main function still has some sloppy code but it is very functional and only written once as opposed to the nightmare of an if statement in the old version. This version uses a function called roller() which can roll any type of dice any number of times. This was the first thing implemented in this version which the old version sorely lacked. Next problem was I needed to find a way to match the second argument with an assocated number to feed into roller(). This is done by the matcher() function. This was working great but a new problem arose. How was I going to feed roller() an atypical dice? The way the program worked then was the user would enter the program name (dice_simulator) followed by two arguments, the amount of dice being rolled and what type of dice to roll. The standard format would look something like this...

dice_simulator 5 d6

The program would then spit out each individual roll followed by the total of the dice added up. If the user entered something like...

dice_simulator 5 d30

It would error out.
That's when the word_split() function came into existence.
This function broke the second argument into bytes and searched through them taking out the "d" in front of the number. It would then return the number as an integer and feed feed it back into roller. The code is not quite my own as I copied it from a book called "The Rust Programming Language" second edition. That chapter was about ownership and it was showing an easy way to separate words from each other using the spaces as a delimiter. I figured the code would fit perfectly here with a few small modifications. The original 178 lines of code was shortened to 63 lines. Quite the improvement.   

### **Christmas**
This was a practice problem in "The Rust Programming Language" book. It was after chapter 3 where it recommended trying to solve a few different problems. The question simply read "Print the lyrics to the Christmas carol "The Twelve Days of Christmas" taking advantage of the repetitions". This was the solution I came up with and it surprisingly worked the first try. The lyrics() function simply returns the appropriate lyric when given the specific number. The looper() function then will print the lyrics in descending order. 
