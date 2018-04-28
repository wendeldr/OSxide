# Dominic Farolino

Final assessment, spring, 2018.

## Part A

Given the breadth of the project, the time constraints, and the experience levels of
all of our team members, the project was a great and ambitious learning experience. I
played a breadth of smaller roles in the project accomplishments. In the beginning I was
most interested in the security guarantees provided by the rust compiler and how it might
relate to what we can expect to see. This more-clearly helped us define our requirements
and where exactly the threshold would be, in regards to how much security we'd have implement
manually (and not rely on the compiler for). Furthermore, I helped setup some of the initial
issues/labels on GitHub for collaboration.

I helped the team do some research on previously existing embedded rust projects for inspiration,
and worked with the team on first getting some code running on the board, which was our biggest
obstruction. I helped with the direction and implementation of the kernel, and researched information
about the formal verification efforts of Rust to present during the design expo.

## Part B

Our group accomplished creating a relatively simple RTOS kernel written in the rust programming language. We
felt this ws pretty special, because rust is a pretty new language in the embedded world and in general, so
there was not much community consensus as to whether it was ready for this realm. We were happy to contribute
some secure embedded code using rust, furthering the amount of work and assessment that has been done in this field.
We learned *a ton* about hardware and embedded programming which was really awesome. I think we each learned a lot
more than we expected, just by diving into some of the code at a level of granularity we've never seen before and
trying to just get things to run.

I learned that a non-negligable of working on a big team project is planning/finding times to meet up. This I think
is often overlooked, but is crucial to getting progress and staying synced up. It seems taken for granted, the communication
and synchronization aspect of team work but I think mastering it is difficult and can have a big effect on the result of
the project. I think Doug Flick deserves special recognition for his work on the **N**ested**V**ector**I**nterrupt**C**ontroller,
and other BSP/low-level related things. He did a ton of work on hardware-peripheral-specific things that allowed
the code in main kernel file to run properly. He took a keen interest in the hardware side of things (he vocalized
this clearly as he says he regrets doing CS over CE @ UC) and that was a really big help because most of us had not
had any previous embedded/hardware experience.
