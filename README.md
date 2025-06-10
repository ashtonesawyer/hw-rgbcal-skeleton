# rgbcal: RGB LED calibration tool
Bart Massey 2024  
Forked by Ashton Sawyer 2025

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held.

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

## Calibration

```
Red   - 15
Green - 11
Blue  - 9

FPS   - 60
```

To diffuse the LED I placed a white paper napkin directly
(touching) over the LED. The farther the napkin was from the LED, the
more the colors separated and the harder it was to tell how white it 
was in the aggregate. 

The above color levels were eyeballed, but given the direct comparission
I was able to do with the napkin, I think they're pretty close. It is
likely that different levels at a similar ratio would produce
a similar white. 

The minimum frame rate seems to be 60fps. At 50fps the light looks 
stable when looking at it straight on, but flickering can be seen
when looking at it from the corner of your eye. 

## Development
### rustdoc
I've never used rustdoc before, so I don't know the conventions for what
should or shouldn't be commented about. I ended up just writing the
comments that made the most sense to me personally. If I were shipping
any of the components as libraries for other people to use, I likely would
have more aggressive commenting. 

### Frame Rate
The first thing that I did after understanding the code better was to 
connect the knob to the frame rate. I decided that it made the most sense
to handle the frame rate similarly to how to LED levels are handled. This
meant creating a global variable and writing a setter and a getter. 

I had a little bit of a hard time getting the syntax for my setter correct
because I made it more complicated than it needed to be, assuming that 
the code would look different from what was provided because I was working
with an integer rather than an array. Other than the initial familiarization
with the code, this was the hardest part of the assingment. That says more
about the difficulty of the assignment than the difficulty of the problem.  

### RGB Levels
Once the frame rate was implemented, setting the levels for all of the
colors was simply a matter of repurposing the provided code that set the 
levels for blue. 

I'm quite pleased with my use of the match statement. My first impulse was
to use if statements and I kept being frustrated by how messy they looked;
checking both variables felt redundant, but only checking one didn't seem
readable enough. Not only is the match statement cleaner in terms of the
conditions it's checking, it also allowed me to only check if the button
was pushed once without creating additional local variables (I don't know
how the statement is implemented, so it may be that in practice temporary
local variables are created, but I prefer the visual of not having them
in my code). 

### Jankiness
There are two major things that I have greatly considered changing:

1. Better state inialization

Right now the LED is set to its default levels on initialization and the
frame rate is set to whatever the knob happens to be on when the program
is started. Part of my feels like either both should be set to defaults,
or both should be set to the knob for consistancy's sake. This doesn't seem
like a big enough deal to bother changing, especially since the initial
state is changed so quickly

2. (more importantly) The knob doesn't detect whether it's moving, just 
whether it's *been moved* since it last polled the currently selected attribute

This means that parts of the state change even when the user isn't actively 
adjusting those levels.

To give an example, say you want to set the frame rate to 100 and blue to
level 3. If you change the frame rate first, the knob will be set to level
9\. When you press the button to change blue's level, it will jump from 
wherever it was to level 9. Then you move it down to level 3. When you
release the button, the frame rate will jump to 40 and will have to be
adjusted again. 

This forces an order of operations on the user, where they have to start by
changing the most restricted attribute and end by changing the least
restricted attribute:

```
Order of operations: 
    1. Red
    2. Green/Blue
    3. Frame rate. 
```

I feel relatively okay not fixing this because the use case is an internal
tool for infrequent calibration. The UI being slightly less user friendly
feels acceptable as long as it's consistant and satisfies the intended use,
which it does. 

