Title: Problem Set 1 Answers
Author: Andrew Becker

Problem 1

User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/31.0.1650.63 Safari/537.36

I'm using an linux distribution that uses an x86 64 bit architecture, which explains the Linux x86_64
Used Chrome to open http://localhost:4414, which would be why that Chrome information is there.

Problem 2

Since you're dealing with threads here, you are dealing with concurrent access to the global variable, which could cause inaccuracies if 2 threads try to increment it at once, or you are trying to write to and read from that location at the same time.