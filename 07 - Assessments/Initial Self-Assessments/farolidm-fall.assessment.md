# Dominic Farolino

Initial self-assessment, Fall semester, 2017.

## Application of academic coursework

The content provided in several courses throughout our academic career will become relevant in a practical setting. Specifically
these courses are:
 - Introduction to Computing Systems
 - Data structures
 - Operating System Concepts
Understanding how the CPU processes instructions is extremely important for the low-level programming required at the operating system
level. Furthermore, the kernel will need to implement many basic data structures from scratch that higher-level processes can take advantage
of. All of this should happen while providing a process scheduling model that is secure by default. In other words, it should be impossible
for application code to have the level of control over the computer hardware that the kernel has.

## Application of co-op experience

My first two co-ops at Interactive Intelligence did very little to provide a practical environment for me to practice some
of the low-level theory I had gained in the classroom. Luckily my third co-op with Mozilla provided me with a great opportunity
to grow in this area! While I personally have very little real world operating systems experience on my own, the co-op at Mozilla
has taught me much about low-level programming that I had not experienced in the classroom. Firefox must be efficient, and therefore
interfaces directly with the OS API required to build efficient applications that interface directly with the OS API. I learned a lot
about the general compilation process of software, how to write multiprocess applications in C++, and how system calls work within the
kernel while providing application security. Besides allowing me to both practice and gain real world experience in this area, this co-op
experience gave me the hunger for more of this experience that drives much of the passion I have for this senior design project.

## Motivation for project

There is plenty of motivation for the type of project we’re undertaking. Chief among many OS concerns is security, which operating systems
written in the C programming language traditionally have many issues with by default. Rust is a systems programming language that promises
a higher level of security than C does, leading to inherently secure programs by default. We intend to put the Rust programming language to the test by attempting to create an operating system free of many of the security vulnerabilities that C-based operating systems have out of the box, and verifying the supposed security features that Rust promises.

