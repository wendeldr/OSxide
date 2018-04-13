# O<sup>s</sup>xide: An RTOS for Secure IOT in Rust

This repository serves as a place for the documentation of the O<sup>s</sup>xide Project. O<sup>s</sup>xide is a real time operating system written in Rust that aims create a foundation for secure Internet of Things (IOT) applications. 


1. [Abstract](#abstract)
2. [Project Description](#project-description)
3. [User Stories and Design Diagrams](#user-stories-and-design-diagrams)
   * [User Stories](#user-stories)
   * [Design Diagrams](#design-diagrams)
   * [Diagram Descriptions](#diagram-descriptions)
4. [Tasks and Timeline](#tasks-and-timeline)
   * [Task List](#task-list)
   * [Timeline](#timeline)
   * [Effort Matrix](#effort-matrix)
5. [ABET Concerns](#abet-concerns)
6. [Slideshow](#slideshow)
7. [Self-Assessment](#self-assessment)
8. [Professional Biographies](#professional-biographies)
9. [Budget](#budget)
   * [Expenses](#expenses)
   * [Donated Assets](#donated-assets)
10. [Appendix](#appendix)


## Abstract
The aim of our project is to create a real-time operating system (RTOS) utilizing rust.  The RTOS will operate on a specific set of microcontrollers with hopes they can be utilized in Internet of Things (IoT) devices.  The primary goal of the RTOS is to create a responsive and secure environment for IoT operations.  Rust has been chosen due to its concurrency capabilities and its emphasis on safety.  Initial goals are to get the RTOS running.  Later small example applications will be developed for the RTOS.

## Project Description

#### Team:
- Codi Burley       : burleycj@mail.uc.edu
- Daniel Wendelken  : wendeldr@mail.uc.edu
- Douglas Flick     : flickdm@mail.uc.edu
- Dominic Farolino  : farolidm@mail.uc.edu

#### Advisor: Dr. John Franco

#### Background
Real Time Operating Systems (RTOS) aim to serve as operating systems that satisfy strict time and reliability constraints of some real time applications. Some RTOSs that have been popular in the past include RTLinux, VxWorks, eCos, QNX, etc. These and most RTOSs have historically been built using the C programming language.

While C has been the long time favorite in the types of low-level programming that is often needed for creating an operating system, a new low-level language with the name Rust is gaining popularity. Rust is a modern systems level programming language that promises speed and security. It provides protections against common system level vulnerabilities including use-after-free attacks, buffer overflow attacks, and many more. This makes Rust a good choice to build a modern, secure, RTOS. An area that a secure RTOS may be used is the growing field of internet of things (iOT).

iOT is a movement towards the connection of physical devices in a way that allows information/data to be shared and utilized by all these “things”. These physical devices are often embedded systems that interact with different types of sensors or other means of gathering data, and they often share this data over some network to other physical devices. Because these are embedded systems, they often utilize RTOSs, and because they must share this data with outside devices, security becomes a priority.

#### Problem Statment
As hinted in the background iOT is becoming increasingly widespread.  In the past few years we’ve seen attacks like the Mirai Botnet, TRENDnet security camera vulnerabilities, and countless security issues with many devices.  Many of these vulnerabilities are produced from the rapid development and cutting edge nature of iOT devices; however, they would often be negated if a standard RTOS existed for use across devices.

#### Inadequacy of current solutions to the problem
There are many other RTOSs which exist but only two that utilize Rust, zinc.rs and Tock.  There are issues with these implementations zinc.rs is more of a hardware abstraction layer then a full RTOS and Tock is currently developed specifically for Cortex-M microcontrollers.

#### Background skills/interests applicable to problem
Knowledge of OS kernels, embedded software programming, driver knowledge.

#### Our Team's Approach
We hope to be able to produce a Rust RTOS with some basic drivers (usb, network) which would run on a wider range of chips supporting llvm compilation.

## User Stories and Design Diagrams

### User Stories
1. As a IOT Developer, I want a secure operating system that handles real world events in a known time frame.
2. As a Developer at a Defense Company, we need a real time operating system that would be difficult for enemies of state to penetrate.
3. As a Developer at a conveyer belt company, we need to quickly sort items and ship them to a desired location as quickly as possible.
4. As a Developer at aerospace systems company, we need planes that can handle environmental effects as fast as possible and safer from human programming errors.

### Design Diagrams
The design diagrams are split into three different diagrams, each diagram at a higher level of abstraction than the next.

#### Diagram 1
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/04-user-stories-design-diagrams/D01.png "Design Diagram 1")

#### Diagram 2
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/04-user-stories-design-diagrams/D02.png "Design Diagram 2")

#### Diagram 3
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/04-user-stories-design-diagrams/D03.png "Design Diagram 3")



### Diagram Descriptions

The first design diagram describes, at the highest level, the world in which our RTOS kernel will operate in. We have several sources
of input for our operating system. The system itself consists of an OS kernel running on some basic hardware, and can produce two
different kinds of output: output visible and usable to users of the operating system, and generated output in the form of files such
as text files, binaries, images, etc.

The second design diagram elaborate a bit more on the operating system itself. We break the operating system into three parts:

 - Applications: this consists of user and kernel applications that run inside the kernel. These applications (processes
   in the kernel) are managed by algorithms in the kernel.
 - RTOS Kernel & BSP: The kernel itself consists of the RTOS kernel code as well as the board support package (BSP) that ties
   the kernel together with the hardware of the computer. The BSP contains a small bootloader to load the beginning and essential
   portion of the kernel into memory, which in turn finishes loading the rest of the kernel into memory.
 - Hardware: Everything runs on the hardware, which in the context of our project, will consist of a basic board containing a
   Nordic Semiconductor chip.

The third design diagram elaborates on how the kernel will manage applications. Any operating system kernel, at its core, consists
of algorithms and data structures to allocate and schedule processes in order to give them CPU time. Efficient and correct CPU
scheduling is crucial in a RTOS as processes must be guaranteed to finish by a certain time. The memory management portion of the
kernel is responsible for allocating processes to memory such that they can be scheduled by the aforementioned process scheduler.
Another crucial part of a RTOS is the associated timers and clocks which help us maintain the time constraints and boundaries in
which prioritized processes must complete.

## Tasks and Timeline

### Task List
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/05-task-list/task-list-img.png "Task List")


### Timeline
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/05-task-list/task-list-img.png "Task List Timeline")

### Effort Matrix
![alt text](https://github.com/CodiBurley/senior-design-group/blob/master/05-task-list/effort-matrix.png "Task Effort Matrix")


## ABET Concerns

#### Social
Information Security is a pressing concern in our technical society in which internet of things (iOT) technology is being used at an exponentially increasing rate. The average consumer of iOT devices could benefit from our operating system that pushes in the direction of secure iOT applications. If iOT devices are built on top of an operating system that promises a more secure foundation, people may be more willing to adopt some iOT applications that could benefit society. If confidence in iOT security is raised, we may see an increase in iOT that is built into infrastructure which could greatly increase the quality of living. It is hard to say in what ways iOT will be used in the future, but it is clear that the possibilities are endless. By creating a real time operating system that attempts to cement a firm foundation for secure iOT, we can help to pave the way for iOT applications to be used to their full potential with a confidence in security.


#### Economic
While our senior design project will rely primarily on self generated code there are some economic limitations that are imposed. To begin we are constrained to the hardware we choose. In this choice we will be required to purchase a microcontroller which fits our projects needs.  This purchase will need to be self funded and more than one microcontroller may be needed to facilitate development.  Another constraint imposed on us is the use of other pre-existing RTOS codebases for reference or code-reuse. Many of these are licensed for free use but care will need to be taken to reference properly.  Overall our project should not have a large economic impact to the team or the users of the created RTOS. 


#### Security
A real time operating system is not designed to be general purpose, such as a phone, laptop, or desktop computer. That being said, a real time operating is designed to provide a deterministic execution pattern. These types of systems may be used by medical devices, wearable technology, assembly lines, power plants, and many more sensitive areas. In all of these cases criminals and foreign agents may attempt to exploit the system to cripple a company or governments infrastructure or do harm to individuals. Our project attempts to provide security by default. The system will be written in the programming language called rust where the focus is on preventing many of the simple errors that are commonly exploited and by preventing segmentation faults and guarantees thread safety. Overall a core part of our project is designing our real time operating system to prevent attacks and data loss.

#### Legal
It is true that when designing any non-trivial piece of software, legal concerns come into play at a regular pace. One such legal concern follows from the idea that we’re only using free and open software, and no intellectual property or copyrighted software in an invalid manner. Given the preliminary research we’ve performed, just about all of the software we’ll be referencing is open-source under an MIT license, which provides us no guarantees regarding the software we’re using in exchange for the ability to use and reproduce it in anyway we’d like. Another legal concern for the project that has the potential of cropping up much later in the future revolves around whether we’d like to provide security guarantees around our software for industry usage. Most modern day operating systems have an EAL (Evaluation Assurance Level) of 2 to 3, which is a security assessment ranking assigned by the NSA after thorough testing. The Real Time Operating Systems in charge of marshalling commands in high-risk situations are often required to have an EAL level of 5 or 6 (7 being the highest) to be considered justified. If one day, we wanted our RTOS to compete on the market for legal industry contracts, it would be in our best interest to shoot for an EAL level of 5 to 6, meaning the core security design in our RTOS would have to be rock solid. This is something to consider in the future if we were looking to have our RTOS be legally involved in high-risk situations, and is for now not a forefront concern.



## Slideshow
This link provides a presentation on the project: https://youtu.be/QamIhC8fg7M

## Self-Assessment
Below are some self-assements made by the team members of the project

### Codi Burley
Throughout my time as a student in computer science and  a worker in the software, I’ve seen many new technologies revolutionize the way certain industries are done. This is especially apparent in the web development community, where a huge range of new tools and frameworks are developed and adopted quickly and on a regular basis. When it comes to systems level programming, however, old technologies that have historically been used in an unsafe manner are still utilized. By aiming to create a secure real time operating system in Rust, I feel that I’m giving myself the opportunity to adopt a new technology in an area that I don’t have much experience in: systems level programming. This project will give me the chance to improve my skills in an area I know little about while attempting to show that new technologies can be used in systems level programming in order to create more secure real time systems, in a time when more and more real time systems are becoming connected to the outside. My hope that is by the end of the project, even if our operating system isn’t complete, we have still shown that Rust is a technology that can and should be utilized for similar projects in the future.

As a computer science student, there have been many skills I have acquired that are applicable to this senior design project. General concepts in computing, and specifically programming, were taught in the beginning of our curriculum (in Computer Science I and Computer Science II). These skills apply to almost all fields of computer science, and helped to build up the knowledge needed in order to complete some of the more advanced courses. One of the more advanced courses I took that applies to this project was Programming Languages. Because Rust is a new language, with new constructs, learning how to utilize and design programming languages has helped greatly in understanding how to learn and use Rust. Another course that will help in my contributions to this project is Operating Systems. In this course we learned the responsibilities and behaviors of many different types of operating systems. It's easy to see how a course like this would be applicable to a project that relies on the development of an operating system. 

Much of my knowledge accumulated while at UC has come not only from the Computer Science curriculum, but also from co-op experience. During my time at GE Aviation as a Design Engineer, I learned the importance of redundancy and the importance of the verification of your work. It’s vital that the components of a jet engine are verifiably safe, and this is something that will be important in making a secure system like we intend to do for this project. At Kinetic Vision, I served as a software developer, and learned many good practices for managing a large software project that is in collaboration with many people. The operating system that we intend to create will likely include many components and a very large code base, which will require that the team can collaborate in a manner that is conducive of productivity. My last co-op was at NASA Ames as a Research Engineer. There I learned how to address a problem that does not have a definite answer, and how to do research in general, which will clearly help out this project. Additionally, the software I wrote at NASA was built on top of a real time operating system. This gave me some experience with how a real time operating system works.

It is a reasonable question to ask why I chose to work on a project that requires writing an operating system, when I don’t in fact have any experience in writing an operating system. In the past I have been very interested in programming languages, and I have always leapt at the opportunity to apply something new in order to try it out. Rust is a language that is new to a field of programming that hasn’t had many revolutions as of late, and that gave me motivation to attempt to use it in our project. As of late, I have also been interested in cyber security, along with the members of my group. This gave us a reason to look for a project that could contribute to that field. By choosing a project that requires that we write an operating system, I’ve also given myself the chance to learn how to write a kernel which is something I’ve always wanted to do, and I’m excited to see what kind of results we can get.
	
To get started on the project, we plan to utilize the previously written real time operating systems. Through analyzing what other real time operating systems got right, and what they did wrong, we can use open source code that will be useful, and find spots we can improve on. Because writing a real time operating system is a hefty task, it is not fully expected that our operating system will be “production ready” by the end. We expect to have an operating system that is functional, with maybe only a few necessary drivers. We hope to show that Rust can be utilized in order to create an operating system that is safe through the utilization of safe constructs in Rust. The evaluation of our project should come through the comparison of our operating system to other real time operating systems in Rust. We hope to show that our projects against some vulnerabilities that others have not.


### Dominic Farolino
#### Application of academic coursework
The content provided in several courses throughout our academic career will become relevant in a practical setting. Specifically
these courses are:
 - Introduction to Computing Systems
 - Data structures
 - Operating System Concepts
Understanding how the CPU processes instructions is extremely important for the low-level programming required at the operating system
level. Furthermore, the kernel will need to implement many basic data structures from scratch that higher-level processes can take advantage
of. All of this should happen while providing a process scheduling model that is secure by default. In other words, it should be impossible
for application code to have the level of control over the computer hardware that the kernel has.
#### Application of co-op experience
My first two co-ops at Interactive Intelligence did very little to provide a practical environment for me to practice some
of the low-level theory I had gained in the classroom. Luckily my third co-op with Mozilla provided me with a great opportunity
to grow in this area! While I personally have very little real world operating systems experience on my own, the co-op at Mozilla
has taught me much about low-level programming that I had not experienced in the classroom. Firefox must be efficient, and therefore
interfaces directly with the OS API required to build efficient applications that interface directly with the OS API. I learned a lot
about the general compilation process of software, how to write multiprocess applications in C++, and how system calls work within the
kernel while providing application security. Besides allowing me to both practice and gain real world experience in this area, this co-op
experience gave me the hunger for more of this experience that drives much of the passion I have for this senior design project.
#### Motivation for project
There is plenty of motivation for the type of project we’re undertaking. Chief among many OS concerns is security, which operating systems
written in the C programming language traditionally have many issues with by default. Rust is a systems programming language that promises
a higher level of security than C does, leading to inherently secure programs by default. We intend to put the Rust programming language to the test by attempting to create an operating system free of many of the security vulnerabilities that C-based operating systems have out of the box, and verifying the supposed security features that Rust promises.

### Daniel Wendelken
We plan to create a real time operating system (RTOS) using Rust. Rust is a low level programming language with some built in safety nets that prevent segfaults and thread safety. Real time operating systems are reliable in both security and performance in a given time-frame. The combination of the Rust and a RTOS leads to a robust and secure os.  This os can be utilized by inherently vulnerable systems like IOT devices, vehicle systems, and other embedded systems.  With the prevalence of these systems growing more secure solutions are needed.

The course most relevant to this project EECE4029 Operating Systems and Systems Programming. In this course we discussed the different layers of a computer system such as the kernel, the architecture privilege rings, and networking.  Additionally we discussed concepts like paging and threading which are all relevant to creating a RTOS. A second course that be useful to this project is EECE2093C Software Engineering.  This course introduced life cycle models and requirement analysis which will be useful in the creation and execution of the project.  Some of these concepts include user stories and black-box/smoke testing.

The only technical work related to this project comes from my co-op experience at the EPA.  Rust shares a lot of similarities with C++ and while at the EPA I briefly worked on a C++ application. Some non technical skills built at my other co-ops which will be useful to this project is the software management sytsem 'agile' and interpersonal skills.  While I was at Kroger we used both kanban and scrum to plan our projects.  Adapting these strategies to our project will help us complete it in an organized and timely manner. The interpersonal skills I've gained from my time at Kinetic vision will help with team communication.

This project is interesting to me for several reasons. First, I've always had an interest in os operations and building a small RTOS will help me clarify and learn more about the internals of an os. I've wanted to learn about kernal development so having to develop it ourselves will force that learning.  Additionally I've recently been getting into computer security so trying to focus the os to be as secure as possible while maintaining a certain level of productivity should be interesting. While I have a base understanding of some security concepts working on this project should solidify my understanding.

Building an os from scratch is no small task. In order to complete the project I think it would be best if we first focus on porting a small preexisting RTOS into rust and work from there. There are several fully open source RTOSs that exist;  The most well known of these is FreeRTOS. The core components of this are ~ 13,000 lines of code and it is written in C. I believe we should begin by porting over the core components of this RTOS into Rust, which is ~ 4,000 lines per group member.  This will give us a decent launching point to develop any further.  To consider this project complete I would like to have a working codebase which can compile on LLVM microprocessors.


TODO Doug add your self assessment


## Professional Biographies

Links to Professional Biographies:
- [Codi Burley](https://github.com/CodiBurley/senior-design-group/blob/master/01-bios/codiburley.md)
- [Dominic Farolino](https://github.com/CodiBurley/senior-design-group/blob/master/01-bios/domfarolino.md)
- [Douglas Flick](https://github.com/CodiBurley/senior-design-group/blob/master/01-bios/douglasflick.md)
- [Daniel Wendelken](https://github.com/CodiBurley/senior-design-group/blob/master/01-bios/wendeldr.md)

## Budget
Currently our budget is $0.  The team is hoping to get ~$200 of funding to purchase a microcontroller for each team member to utilize.

### Expenses
4x | Nordic Semiconductor - nRF51 DK (or similar device) | ~ $44.68 per unit

### Donated Assets
No donated assets

## Appendix

### Research

#### Tock OS - https://github.com/helena-project/tock
Our team took the time to research another real time operating system written in Rust: Tock OS. This is an open source project with a great community, meaning that we could get a look a the code. This gives us crucial insight, as there is not that many resources related to writing operating systems in Rust, because it is a newer technology.

This paper from the Tock OS team also was very useful in understanding the motivations for an RTOS:
Nilsson, F., & Adolfsson, N. A Rust-based Runtime for the Internet of Things.

#### Memory Paging Implementation - https://os.phil-opp.com/page-tables/
Our team also took the time to research how we might implement the paging memory allocation technique in our
Rust-based OS kernel. This page discusses how a similar OS kernel also written in rust, implemented paging. This
project (and this post specifically) gives us a lot of information as we weight the cost of implementing such a
technique.

#### Real-Time System Scheduling Algorithms - http://research.cs.queensu.ca/home/akl/techreports/scheduling.pdf
During our initial research phase we referenced the above paper to get an idea of how real-time systems differ
in scheduling from traditional systems. This is especially important as most of our cumulative experience revolves
around traditional kernels, so getting an idea of what scheduling techniques are efficient and feasible to implement
within the space of a semester is really important.
