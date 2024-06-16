# Bit Manipulation 

 Bit manipulation is the act of algorithmically manipulating bits or other pieces of data shorter than a word. It involves applying logical operations (such as AND, OR, XOR, NOT, etc.) to individual bits or groups of bits in a binary number to achieve a desired resul


### Keys points to be noted

1. It is a low-level technique that directly works with the bits representing data in computer memory

2.  Bit manipulation can be faster than other operations like division, multiplication, and branches as it uses single-cycle CPU instructions

3.  It is commonly used in tasks like low-level device control, error detection, data compression, encryption, and optimization

4. Bit manipulation can reduce memory usage and speed up operations by avoiding loops over data structures, as bit manipulations are processed in parallel

## Operators in Bit Manipulation

 -   Right Shift Operator  ">>"
   - Left Shift  Operator  "<<"
   - NOT "!"
   - XOR "^"
   - OR  "|"
   - AND "&"
  
  <img src="https://embeddedwala.com/uploads/images/202206/img_temp_62af09c374d866-36338888-48712239.gif"></img>
### Time and Space complexity

    Constant complexity - O(1)


### Set and Clear bit methods
  - #### Set method is used to set the bit at a particular position(say i) of the given number N. The idea is to update the value of the given number N to the Bitwise OR of the given number N and 2i that can be represented as (1 << i). If the value return is 1 then the bit at the ith position is set. Otherwise, it is unset.
  - #### Clear method is used to clear the bit at a particular position(say i) of the given number N. The idea is to update the value of the given number N to the Bitwise AND of the given number N and the compliment of 2i that can be represented as ~(1 << i). If the value return is 1 then the bit at the ith position is set. Otherwise, it is unset.
     
## Sources for Reference:
   #### - [Wikipedia](https://en.wikipedia.org/wiki/Bit_manipulation)
   #### - [GeekforGeeks](https://www.geeksforgeeks.org/all-about-bit-manipulation/)
   #### - [StackOverflow](https://stackoverflow.com/questions/2096916/real-world-use-cases-of-bitwise-operators)
## Real World Examples
  1. Encryption and Data Compression: Bit manipulation is heavily used in encryption techniques like Exclusive-Or Encryption and data compression algorithms. It allows for extracting and manipulating data at a bit level, making it crucial for secure data transmission and storage
  
  2. Low-Level Device Control: Bit manipulation is essential for tasks like controlling hardware devices at a low level. It enables programmers to interact directly with hardware components by manipulating individual bits, ensuring precise control and efficient operation
   
  3. Error Detection and Correction: Bit manipulation plays a vital role in error detection and correction algorithms. By working at the bit level, programmers can implement efficient error-checking mechanisms to ensure data integrity and reliability
   
  4. Optimization: Bitwise operations are faster and can optimize programs effectively. By leveraging bit manipulation techniques, programmers can enhance the performance of algorithms and optimize memory usage, leading to more efficient and streamlined code
   
  5. Competitive Programming: In competitive programming, bit manipulation is a valuable skill. Interviewers often include problems related to bit manipulation, such as bitmask dynamic programming and number of subsets, to assess a programmer's understanding of bitwise operators and problem-solving abilities

### Count Ones 

<img src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/setbit.png"></img>

<p>Simple Method Loop through all bits in an integer, check if a bit is set and if it is, then increment the set bit count.</p>

<h3>Time Complexity</h3>

#### Complexity - O(log N)