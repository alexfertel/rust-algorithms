## Ciphers

### [Transposition](./transposition.rs)
In cryptography, a **transposition cipher** is a method of encryption by which the positions held by units of plaintext (which are commonly characters or groups of characters) are shifted according to a regular system, so that the ciphertext constitutes a permutation of the plaintext. 
That is, the order of the units is changed (the plaintext is reordered). 
Mathematically a bijective function is used on the characters' positions to encrypt and an inverse function to decrypt. 

In this case, the implementations works as following:

For each character of the keyword string a new column inside a table is created. 
Each column receives the corresponding character of the keyword string.
Every character of the input string will then be put in the fields from left to right. 
Empty fields will be filled with the character 'X'.
The keyword string and its corresponding column is then sorted by its alphanumeric values. 
To get the encrypted String every character inside the table will be added from
top to bottom and left to right.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Transposition_cipher)
