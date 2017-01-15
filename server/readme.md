# Fig Table Server

There were 3 possible designs I could have gone with in designing this server:

## Design 1

Manipulate the GPIO pins at the lowest possible level via `memmap` to map to the physical address locations that the MMU uses to transfer data to and from the GPIO pins.

## Design 2

Use the user space functionality of just writing to `sys/class/pwm`. 

## Design 2

Use the [wiringPi](https://projects.drogon.net/) library, compile it on the Pi, and use the CLI to communicate with the GPIO pins.

## Design 3

Use the python wrappers for the GPIO pins to manipulate them and rewriting the server in python, and installing python on the pi. 

---

I opted to use design 2 since this offer the right level of control while at the same time saving me from the headache of having to figure out which address I'm supposed to be dealing with, memory mapping, etc. This is also far more portable for future Pis.

