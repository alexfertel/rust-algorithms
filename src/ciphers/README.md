## Ciphers

### [Caesar](./caesar.rs)
![alt text][caesar]
In cryptography, a **Caesar cipher**, also known as Caesar's cipher, the shift cipher, Caesar's code or Caesar shift, is one of the simplest and most widely known encryption techniques.<br>
It is **a type of substitution cipher** in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet. For example, with a left shift of 3, D would be replaced by A, E would become B, and so on. <br>
The method is named after **Julius Caesar**, who used it in his private correspondence.<br>
The encryption step performed by a Caesar cipher is often incorporated as part of more complex schemes, such as the Vigenère cipher, and still has modern application in the ROT13 system. As with all single-alphabet substitution ciphers, the Caesar cipher is easily broken and in modern practice offers essentially no communication security.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher)

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

[caesar]: https://upload.wikimedia.org/wikipedia/commons/4/4a/Caesar_cipher_left_shift_of_3.svg
