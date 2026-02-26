# Bitcoin Client Project, Assignment 1

# Part 1

Hi, welcome! This is your first assignment in this course. We will use Rust throughout this course.

## Introduction

We expect you to have some programming experience and are familiar with at least one programming language. If you don't know Rust language, it is totally okay, since in this course you'll self-teach Rust language by reading the documents of:

- the Rust language itself;
- Cargo, the rust package manager, and building tool;
- Rust standard libraries;
- Rust crates.

You will finish simple tasks in the codebase. It divide the project into several sub-projects/assignment, each assignment may include several smaller parts. This is the first part of the assignment.

In this course project, you will build a simplified Bitcoin client. The client's goal is not to run in the Bitcoin mainnet or any public testnet. Instead, the goal is to run on your own machine and let you have fun with it. You have plenty of freedom in designing and implementing this project.

## Requirement:
1. You will work on the project individually.
2. You can run tests (by command `cargo test`) provided in the code to check the validity of their implementation. However, passing these tests doesn't guarantee to get full grades. 

## Reading 

Please refer to Rust by example (can be easily search and found on google), to learn Rust grammar.

Please refer to Cargo documentation (can be easily search and found on google) to learn Cargo, the Rust package manager, and building tool. After reading chapter 1 (Getting Started), you'll be able to install Rust and Cargo and run a Rust project.

For Rust standard crate, It is recommended that you learn two very important structs: **String** and **Vec**.

You can learn about other public crates on docs.rs (can be easily search and found on google). A *crate* just means a library or a package and can be managed by Cargo. You will learn how to use the following crate:
- ring, a cryptographic crate. Specifically, you need to learn how to do SHA256 hash.
- serde and bincode, serialization crates. Specifically, you need to learn how to encode an object into bytes.

For these crates, their GitHub page or homepage may also be helpful. Feel free to read them.

Feel free to explore plenty of exercises on rust online for practice.

## Code provided
We have provided incomplete code for implementing some crypto-primitives. The following files are related to this assignment.

`src/types/address.rs` - Provides __Address__ struct (20 byte array).

`src/types/transaction.rs` - struct defition of **Transaction** struct and function declaration for __sign()__ and __verify()__ .

As for other files in the src folder, you don't have to worry about them in this part of the assignment. They may appear in later part of assignments/projects.

## Programming
After you download the project zip, it is first suggested to run the command `cargo test` to see whether the code is compiling on your machine. (If compilation has errors, please check if you are running the latest stable version of Cargo.) If the compiling is successful, you will see something like this:
```
running X tests
test XXX ... FAILED
test XXX ... FAILED
```
It's expected that tests fail with the code initially provide. After you finish this part of the assignment, some of the tests will pass. it is encouraged that you to add your own tests to your code as well.

These are the tasks of this assignment:

1. You need to implement the missing parts in file `src/types/address.rs`:

- `fn from_public_key_bytes(bytes: &[u8])`

- It uses SHA256 (from **ring** crate (version >= 0.16.20)) to hash the input bytes, and takes the last 20 bytes, and converts them into a __Address__ struct. The code now contains `unimplemented!()`, and you can delete it and write your own code.

- It provide a small test function named **from_a_test_key()**. After you finish coding, you can run `cargo test from_a_test_key` and see this function's result in the output. It will look like the following.
```
test types::address::test::from_a_test_key ... ok
```
- To test your code, you are free to write more tests.

2. The missing parts in file `src/types/transaction.rs`: 

- Fill in the **Transaction** struct. We don’t expect the cryptocurrency and payment to be functional at this point, so you can put any content in transactions. A simple choice is to put **sender**, **receiver**, and **value** inside transactions. **sender**, **receiver** are of type **Address** and **value** is integer.
- Fill in the `sign` and `verify` functions. These two functions should sign and verify the digital signature of the **Transaction** struct. Please use **ring** (version >= 0.16.20) crate. You can use the bincode crate to serialize and deserialize any struct. The code it provide contains some `unimplemented!()` and you can delete it and write your own code.
- A tricky part about transaction and signature is how you put them together. Hence, we provide another struct called **SignedTransaction**. You can let this struct have a transaction, a signature, and a public key that creates the signature. Notice that crate *ring*'s signature and public key structs may not be very convenient to use, so you can convert them to a vector of bytes: `let signature_vector: Vec<u8> = signature.as_ref().to_vec();`
- For testing, you need to fill in the function **generate_random_transaction()**, which will generate a random transaction on each call. It should generate two different transactions on two calls. We require this since we frequently use this function in our tests and grading. Again, there is `unimplemented!()` and you can delete it.
- It provide a small test function named **sign_verify()**. After you finish, you can run `cargo test sign_verify` / `sign_verify_two` and see this function's result in the output. It will look like the following.
```
test types::transaction::tests::sign_verify ... ok
```
- To test your code, you are free to write more tests.

## Advance Notice
- At the end of the course, you will implement a functional cryptocurrency client based on this codebase. So it is helpful to get familiar with this codebase.
- This code base provides other files that will help you build a blockchain client. If you want to run the main program and see what is going on, you can run `cargo run -- -vv`. Currently, the main program is just stuck in a loop. (`-vv` is for level 2 logging. You can have `-vvv` for level 3.)
- At the end of the project, you will implement a functional cryptocurrency client. In this assignment, we provide a temporary transaction structure that contains sender, receiver, and value. You can think of a transaction struct that can support a real cryptocurrency; also, explore how Bitcoin and Ethereum do this.

# Part 2
In this Part 2, you will implement some crypto-primitives and basic data structures. Please follow the instructions.

Similar to the previous Part, you will continue to work on your project.

## Code provided
The code base include incomplete code for implementing some crypto-primitives and data structures like merkle tree. The following files are related to this part of the assignment, and you should read them.
1. `src/types/hash.rs` - Provides __H256__ struct(32 byte array),  __Hashable__ trait, with its implementation for H256. (You don't need to write anything in this file.)

2. `src/types/merkle.rs` - struct definition of **MerkleTree** struct and the related function declaration. You will write your code in this file.

The other files in the src folder are not relevant to this assignment. They may appear in future assignments/projects.

## Programming
You need to implement the missing parts in the code. They include the following.

### Merkle Tree
This part is in file `src/types/merkle.rs`. You need to complete the Merkle tree struct and some functions. Specifically, the functions you need to implement are:
1. *new()* - This function takes a slice of Hashable data as input and creates the Merkle tree. 
2. *root()* - given a Merkle tree, return the root. The root should be computed in *new()*; this function should just return it.
3. *proof()* - given a Merkle tree, and an index (starts from 0), this function returns the proof in the form of a vector of hashes. The proof must return a list of sibling hashes of the index-specified data point, where there is a sibling at each tree level (It should not include the leaf and root). 
4. *verify()* - given a root, a hash of datum, a proof (a vector of hashes), an index of that datum (same index in *proof()* function), and a leaf_size (the length of leaves/data in *new()* function), returns whether the proof is correct.

The examples in the ring documentation (ring::digest, ring::signature) is a useful reference.

*new()* function can take any Hashable data, however we will test the Merkle tree using inputs of type **H256**. The Hashable trait for H256 is already provided in `src/types/hash.rs`.

If the size of the input to *new()* is not a power of 2, you need the following extra steps to create the Merkle tree :
> Whenever a tree level has an odd number of nodes, duplicate the last node to make the number even.

The code base provided a few simple test functions in this file, and you can run `cargo test`. In these test functions, It also briefly explain the expected computation. 
It is highly recommend that you (learn to) write your own test cases to verify your implementation before submission.

After you finish the programming, you can run `cargo test merkle_root` / `merkle_proof` / `merkle_verifying` to test whether your implementation is working.

We will auto-grade the program using tests similar to the ones mentioned above. We will not test edge cases. It is encouraged that you write your own test cases to ensure that your implementation is correct.

## FAQ

- *What data structure should one use to implement the Merkle tree?* - We recommend avoiding a recursive implementation. You can use any suitable data structure; a simple choice would be to use `Vec<>` or `Vec<Vec<>>`.
- *How do I handle edge cases?* - We will not be testing edge cases. However, here are a few suggestions:
         - When `data.len() = 0`, your program should not panic. `root()` can return a (fake) value like `0x000...0`. `verify()` must return `false`.
         - When `data.len() = 1`, the merkle root will be the hash of the datum.
- *Should I handle invalid inputs?* - Yes; however only valid inputs will be tested during grading.
         - If `index >= data.len()`, `verify()` must return `false`.  `proof()` can return an empty vector.

# Part 3

In this part of the project, you will implement the **Block** struct and the **Blockchain** struct.

Similar to the previous Part, you will continue to work on your project.

## Code provided
The following files are related to this part of the assignment.
1. `src/types/block.rs` - Please finish the **Block** struct and some related functions in this file.
2. `src/blockchain/mod.rs` - Please finish the **Blockchain** struct and some related functions in this file. (You can also split codes into several files inside the directory `src/blockchain/`.)

## Programming

### Block

You need to define a **Block** similar to that in Bitcoin. We require that a block must include:
1. parent - A hash pointer to the parent block. Please use **H256** that it provide.
2. nonce - A random integer that will be used in proof-of-work mining. It is suggested that you using **u32**.
3. difficulty - The mining difficulty, i.e., the threshold in the proof-of-work check. Please use **H256**: since it have provided a comparison function, with which you can write `if hash <= difficulty`. (Proof-of-work check or mining is not required in this part.)
4. timestamp - The timestamp at which this block is generated. (you can use `std::time`)
5. merkle_root - the Merkle root of data (explained below in 6.).

The above fields are also known as **Header**. It is suggested (but do not required) that you create a struct **Header** to include them.

6. data/content - The actual transactions carried by this block. We suggest using a **Vec** of **SignedTransaction**. You have already written the SignedTransaction struct in a previous part of the assignment.

It is suggested (but do not required) that you create a struct **Content** to include the content.

Notice that to create the Merkle root of **SignedTransaction**, you must implement the trait **Hashable** for **SignedTransaction**. This trait should be implemented by serializing it into bytes, then calling SHA256 to hash the bytes.

You need to implement the trait **Hashable** for **Block**. The way to hash a block is to hash **Header** rather than **Content**. So you can first implement **Hashable** for **Header**. When you hash a **Block**, you can directly call the hash function of **Header**. Please make sure you serialize the **Header** before hashing it.

To test and debug, you must implement the function `generate_random_block()`. This function takes the hash of the parent block as an argument. The generated block should contain that *parent*. The *nonce* should be a random integer. You can let the content be empty. So merkle_root should be the Merkle root of an empty input (make sure this is accounted for in your Merkle implementation). As for fields such as difficulty and timestamp, choose whatever you like.

### Blockchain

You need to finish a struct named **Blockchain**, which contains the necessary information of a direct acyclic graph (DAG) and provides functions related to the longest chain rule. The following functions are required:
1. `new()` - Create a new blockchain that only contains the information of the genesis block. Define genesis block by yourself. 
2. `insert()` - insert a block into the blockchain. You can (but not required) make it return struct `Result` to enable error handling when an invalid block is inserted. (We will not deal with invalid blocks in this part)
3. `tip()` - Return the last block's hash in the longest chain. The tip should be computed in the new and insert functions; this should just return it.
4. `all_blocks_in_longest_chain()` - return all blocks' hashes in a vector from the **genesis to the tip**. This function will not be tested in this part and will be used in the future.

#### Storage choice

It is suggested that you use a **HashMap** in the standard crate to store blocks. You can use the hash as the key and the block as the value. This enables you to look up the blocks by hash very conveniently.

You can also store the tip and update it after inserting a block. If, say, your current tip is hash(B1), and you insert a new block B2. You need to update the tip to hash(B2) if and only if the length of chain B2 is *strictly greater* than that of B1.

You may also store the length/height of each block in the blockchain and use it to determine the longest chain. E.g., genensis block has height 0. This step is not required.

You can implement this with persistent storage, such as a database, but this is not the point of this project, and we suggest you use in-memory storage.

#### Thread safety choice

In the future, the **Blockchain** struct will be shared between threads, such as miner and network threads. So this struct must be thread-safe. However, this is not hard to do with lock. **You don't need to worry about it in this part.** You can implement a non-thread-safe **Blockchain** and leave the thread safety problem to future parts.

## Grading and Test your code

If your program works, you will pass the test named `insert_one`. (By running `cargo test`.)

We will use other private tests to grade your submission. We will use the `generate_random_block` function that you implemented for tests.

The tests will insert around 50 blocks into a new blockchain and check whether the tip is correct. The tests contain forking/branching scenarios to check the correctness of your longest chain rule. We encourage you to write your own tests to verify the correctness and avoid losing points.

We will *NOT* call the insert function with invalid blocks. Specifically, we will not insert a block whose parent is not already inserted.

## FAQ
- *Can the fields of Header/Content structs of the blockchain be made public?* - Yes, they can be made public. You can also define a `get` function instead.
- *What values should the fields in the genesis block have?* - Note that the fields (for the genesis block) such as nonce, difficulty, timestamp, parent should be fixed and not random. You can set nonce and timestamp to `0` and difficulty to `0xff..ff` and parent to `0x00..00` (or any other fixed values for that matter).
- *How does one set values to a variable of type H256?* - You can create a `[u8;32]` with fixed values and convert it to H256 using `.into()`. Alternatively, you can use the `hex_literal` crate and use `.into()`.

## Advance Notice
1. If you want to learn about thread safety of the Blockchain struct, you can try `Arc<Mutex<Blockchain>>` in your code.
2. Our goal is to decouple blockchain status from ledger status and focus on the former. As a result, we don't involve transaction execution, ledger update, or UTXO in this part. They will be handled in future parts.
3. We don't use proof-of-work check or mining yet, but we must prepare for them. So we require the fields nonce and difficulty inside a block. You can start to think about how to mine or check blocks.
4. The Blockchain struct will be used in multiple places in the future. For example, when you implement a miner, you insert a mined block into the blockchain; when you want to mine on the longest chain, you need to get the tip as the block's parent; when you receive a block from p2p network, you insert it.
5. We don't require you to put a coin base transaction inside blocks in this part.

## Submission

Compress your project's src folder into a zip file. Rename the file using your specific NetID/loginid (the 5 to 6 alphanumeric code) in this format: `netid.zip`. Upload the file to Canvas under 'Project Assignment 1'.

---
Source: Dr. Pramod Viswanath, Principles of Blockchains (Princeton).