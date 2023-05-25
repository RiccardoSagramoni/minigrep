# minigrep

Source: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

Rust's speed, safety, single binary output, and cross-platform support make it an ideal language 
for creating command line tools, so for our project, we’ll make our own version of the classic 
command line search tool `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint). 

In the simplest use case, grep searches a specified file for a specified string. 
To do so, grep takes as its arguments a file path and a string. 
Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.
