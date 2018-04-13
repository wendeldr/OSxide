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
OSxide is a project that aims to create a real-time operating system (RTOS) utilizing Rust. By Creating a real time operating system in Rust, OSxide contributes embedded Rust code to the community at a time when the amount of public embedded Rust is extremely sparse. Writing the RTOS in Rust benefits from the secure and memory-safe paradigms that the language implements. This is contrary to the many embedded systems that have historically been written in C, which allows many of the unsafe behavior that Rust aims to prevent. In a time when embedded systems are becoming more relevant than ever, through various internet of things (iOT) platforms, it is important that measures are taken to allow for the construction of more secure embedded systems, and OSxide is towards these secure systems.

We have implemented OSxide, an RTOS written fully in Rust that runs on an NRF51 development kit. The RTOS encompasses a basic scheudler that allows for inter-task communication. Thus far, the tasks have interracted with peripherals that are built in to the development kit. Access to the peripherals is allowed through a hardware abstraction layer that was written by our team as well.

## Project Description

#### Team:
- Codi Burley       : burleycj@mail.uc.edu
- Daniel Wendelken  : wendeldr@mail.uc.edu
- Douglas Flick     : flickdm@mail.uc.edu
- Dominic Farolino  : farolidm@mail.uc.edu

#### Advisor: Dr. John Franco

#### Background
Real Time Operating Systems (RTOS) aim to serve as operating systems that satisfy strict time and reliability constraints of some real time applications. Some RTOSs that have been popular in the past include RTLinux, VxWorks, eCos, QNX, etc. These and most RTOSs have historically been built using the C programming language.

While C has been the long time favorite in the types of low-level programming that is often needed for creating an operating system, a new low-level language with the name Rust is gaining popularity. Rust is a modern systems level programming language that promises speed and security. It provides protecns against common system level vulnerabilities including use-after-free attacks, buffer overflow attacks, and many more. This makes Rust a good choice to build a modern, secure, RTOS. However, it is no secret that there are not many examples available of embedded Rust code in the community. An area that a secure RTOS may be used is the growing field of internet of things (iOT).

iOT is a movement towards the connection of physical devices in a way that allows information/data to be shared and utilized by all these “things”. These physical devices are often embedded systems that interact with different types of sensors or other means of gathering data, and they often share this data over some network to other physical devices. Because these are embedded systems, they often utilize RTOSs, and because they must share this data with outside devices, security becomes a priority.

#### Problem Statment
As hinted in the background iOT is becoming increasingly widespread.  In the past few years we’ve seen attacks like the Mirai Botnet, TRENDnet security camera vulnerabilities, and countless security issues with many devices.  Many of these vulnerabilities are produced from the rapid development and cutting edge nature of iOT devices; however, they would often be negated if a standard RTOS existed for use across devices.

#### Inadequacy of current solutions to the problem
There are many other RTOSs which exist but only two that utilize Rust, zinc.rs and Tock.  There are issues with these implementations zinc.rs is more of a hardware abstraction layer then a full RTOS and Tock is currently developed specifically for Cortex-M microcontrollers. It is clear that this a need for more embedded Rust in the community to help push the movement towards writing secure embedded Rust forward.

#### Background skills/interests applicable to problem
Knowledge of OS kernels, embedded software programming, driver knowledge.

#### Our Team's Approach
Our team aimed to create a basic RTOS in order to contribute embedded Rust code to the community. Using inspiration from some open source projects such as [nrf51-hal][https://github.com/therealprof/nrf51-hal] and [cortex_m](https://github.com/japaric/cortex-m), we achieved running Rust on an embedded chip. We then worked towards writing a scheduler and HAL in Rust, and tested the RTOS with some basic tasks that interact with peripherals.
