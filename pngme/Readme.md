![Screen Shot 2023-02-15 at 2 36 58 PM](https://user-images.githubusercontent.com/90811429/219137450-32d34467-90ce-44e7-8cd3-f85c84a81c3f.png)


I want to master the ability to use rust to become, so I plan to build a rust intermediate project step by step, following the tests and ideas provided in this textbook
https://picklenerd.github.io/pngme_book/introduction.html

## Introduction 

We're making a command line program that lets you hide secret messages in PNG files. Your program will have four commands:

- Encode a message into a PNG file
- Decode a message stored in a PNG file
- Remove a message from a PNG file
- Print a list of PNG chunks that can be searched for messages 
- 
If that sounds scary and beyond your ability then this guide is definitely for you. If you know how to write code and you know your Rust basics, you can totally do this. We're not going to implement any sort of image decoding. The part of the PNG spec we're tackling is surprisingly simple.

