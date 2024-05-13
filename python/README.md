# Python version

Inspiration and guide found from here: <https://realpython.com/conway-game-of-life-python/>

Python version used: 3.10.12
Pip version used: 24.0

## Installing and running the program

1. (Optional) Create a new Python virtual environment and activate it by running the following commands:
   1. `python -m venv venv`
   2. `source venv/bin/activate`
2. Install the project withe the following command: `python -m pip install -e .`
3. Run the program with the Glider Gun pattern for 100 generations: `conway -p "Glider Gun" -g 100`

After installation, you can see how to run the program by running the command `conway -h`

## Review

Since I wrote this project using the guide, this review will not be as qualitative as the other language reviews. However, I've used Python a lot before, so this review will mainly contain my thoughts about the language from previous experience.

Python is a very easy language. It's near instant to start writing Python due to the simple syntax, and the core library contains most necessary stuff needed (however pandas and numpy as most of the times needed :D). The white-space sensitive syntax is horrible however, and lack of curly braces drive me nuts at times. The compiler errors are not always useful however, but due to the readability of the code it's not too bad.

Pros:

- Easy to use and read
- Fast to start creating stuff
- Good tooling

Cons:

- No typing
- I hate the white-space sensitive syntax and lack of curly braces
- Compiler errors are not always very helpful

Overall score: 8
