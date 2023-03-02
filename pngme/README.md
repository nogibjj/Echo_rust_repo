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

The small project is based on the creation and parsing of a custom Png. 
The PNG file structure is actually very simple, consisting of a fixed header signature and a combination of multiple chunks, as shown in the following figure
![image.png](https://i.stack.imgur.com/hcIIO.png)

There are many more details, including the definition of chunk_type, chunk_type consists of their respective letters, the difference in case represents the different characteristics of the chunk.

This is my second Rust project to hide a string of information into a png image. The principle is to add a new chunk that has no effect on the display of the image, and parse the chunk to get the hidden information.
Reference link. [https://picklenerd.github.io/pngme_book/introduction.html](https://picklenerd.github.io/pngme_book/introduction.html)

# Data Structur
## ChunkType
```rust
pub struct ChunkType(u8, u8, u8, u8);
```
## Chunk
```rust
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
```
## Png
```rust
pub struct Png {
    signature: [u8; 8],
    pub(crate) chunks: Vec<Chunk>,
}
```
# Example

## Commands

### Encode a secret into a file

```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE>
```

Example:

```bash
pngme encode ./myfile.png sEcr "Hello, this is a PNG file secret"
```

### Decode a secret from a file

```bash
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme decode ./myfile.png sEcr
```

### Remove a secret from a file

```bash
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

Example:

```bash
pngme remove ./myfile.png sEcr
```

### Print chunks

```bash
pngme print <FILE_PATH>
```

Example:

```bash
pngme print ./myfile.png
```
![image.png](https://cdn.nlark.com/yuque/0/2022/png/23135940/1657510398726-6567250f-0007-4daf-9004-fe2b223927ca.png#clientId=u6933692d-b9d1-4&crop=0&crop=0&crop=1&crop=1&from=paste&height=343&id=u1914f7aa&margin=%5Bobject%20Object%5D&name=image.png&originHeight=686&originWidth=1748&originalType=binary&ratio=1&rotation=0&showTitle=false&size=1036080&status=done&style=none&taskId=u2fff4be3-2efe-4dc2-bb0c-8a9798573e0&title=&width=874)
