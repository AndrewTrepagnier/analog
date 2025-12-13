# Linear Algebra Analog Clock

### Motivation

The precision and design of high quality mechanically-driven clocks was always a super fascinating concept to me growing up. I grew to appreciate this more and more throughout my engineering education. 

I find there is no better convincing way of illustrating how awesome mechanical time keeping is than to just explain what cruelty it is exposed to everyday. For starters: There are over 31.5 million seconds in a year, and certain craft wrist watches are hitting sub 8 second / year tolerances driven purely by tiny springs and gears and powered by your wrist motion. This is truly unbelievable when you think about the temperature fluctuations affect thermal expansions within the springs and pins, not to mention the whiplash forces it will feel throughout its life - this is a marvel of engineering. 

### So what's a linear algebra clock?

The linear algebra clock is a (not so aesthetically pleasing) clock face that takes the digital time of the operating system (i.e. 4:17 , or 10:43) and applies mathematical linear algebra-based transformations to convert the digital time to an analog clock face in the terminal. 

It does not use any index mapping or look-up tables to assign the time to the analog clock face. The analog face, itself, is generated using identity matrices and exchange matrices. From there, rotational transformations, determinants, and eigenstructure calculations are applied to place the "M" and "H" denoting the hand placements. 

**Because the mathematics underpinning this project is the focus, I am okay with the clock face itself is not very asthetically pleasing**

Analog Clock Face:
```
Current Time: 4:13:19

Analog Clock Face:
                              12                                 
                            +     +                             
                          +         +                           
                        +             +                         
                      +                 +                       
                    +                     +                     
                  +                         +                   
                +                             +                 
              +                                 +               
            +                                     +             
          +                                         +           
        +                                             +         
      +                                                 +       
    +                                                      M     
  +                                                         +   
 9                                                              3 
                                                                
  +                                                         +   
    +                                                     +     
      +                                                 +       
        +                                             +         
          +                                    H     +           
            +                                     +             
              +                                 +               
                +                             +                 
                  +                         +                   
                    +                     +                     
                      +                 +                       
                        +             +                         
                          +         +                           
                            +     +                             
                                 6  

```

### Why?

No reason in particular. This was a fun way to combine my fascination for clocks with math and Rust.


### How to run it in your terminal

**Prerequisites:**
- [Rust](https://www.rust-lang.org/tools/install) (includes Cargo)

**Clone and run:**
```bash
git clone https://github.com/your-username/analog.git
cd analog
cargo run
```

The clock will display the current system time as an analog clock face in your terminal.

## How it works, mathematically

### Setting up the clock face 

Firstly, we need to assemble to clock face itself before we can populate the hour and minute hand positions. If you imagine a small 3x3 identity matrix like this:

```bash
I = | 1  0  0 |
    | 0  1  0 |
    | 0  0  1 |

```
You may be able to see how this can useful for forming a quadrant of a diamond-shaped clock face. Thus, if we made this a 15x15 identity matrix, where each 1 is a minute interval of the clock, then all we would need to do is apply some exchange transformations and concatenation to form a diamond clock face of 1's and 0's. From there, we can use boolean logic to negate the negative space (zeros) and replace positive space (ones) with stars. 


```bash
                              12                               
                            +     +                             
                          +         +                           
                        +             +                         
                      +                 +                       
                    +                     +                     
                  +                         +                   
                +                             +                 
              +                                 +               
            +                                     +             
          +                                         +           
        +                                             +         
      +                                                 +       
    +                                                     +     
  +                                                         +   
                                                               
 9                                                              3 
  +                                                         +   
    +                                                     +     
      +                                                 +       
        +                                             +         
          +                                         +           
            +                                     +             
              +                                 +               
                +                             +                 
                  +                         +                   
                    +                     +                     
                      +                 +                       
                        +             +                         
                          +         +                           
                            +     +                             
                               6  


```

## Determining hand placement

A clock hand pointing to 12 o'clock can be represented as a unit vector, and any other time position is simply that vector rotated by an angle derived from the time. This means that we if we know that an hour hand makes a 2π (100%) rotation every one hour, then the radian angle conversion is simply θ = (hour + minute/60) × π/6. Similarly, for the minutes: θ = minute × π/30.

From there, we know can determine our hour and minute hand vectors with respect to the 12 o'clock reference vector using a clockwize rotation matrix:

```
R(θ) = | cos(θ)   sin(θ)  |
       | -sin(θ)  cos(θ)  |
```

It is sometimes easier to see with an example:


Example time = 10:43


\begin{align*}
\theta_H &= \left(10 + \frac{43}{60}\right) \cdot \frac{\pi}{6} = 10.717 \cdot \frac{\pi}{6} \approx 5.612 \text{ rad} \approx 321.5^{\circ}
\end{align*}

\begin{align*}
R(\theta_H) &= \begin{pmatrix}
\cos(5.612) & \sin(5.612) \\
-\sin(5.612) & \cos(5.612)
\end{pmatrix}
\approx \begin{pmatrix}
0.782 & -0.623 \\
0.623 & 0.782
\end{pmatrix}
\end{align*}

Now we just apply the rotation to a reference vector pointing at 12 o'clock:
```
direction = R(θ) × [0, 1]ᵀ = [sin(θ), cos(θ)]ᵀ
```

The final position is a **linear combination** of basis vectors with coefficients derived purely from the rotation matrix.

### Brief side note
While not essential to the clock's operation, the rotation matrix has some elegant properties worth noting. The matrix is orthogonal (R × Rᵀ = I), meaning its inverse is simply its transpose—rotating back is free. The determinant equals 1, confirming it's a pure rotation with no scaling or distortion. Most intriguingly, the eigenvalues are complex: λ = cos(θ) ± i·sin(θ) = e^(±iθ). This means the rotation angle, and therefore the time, is literally encoded in the eigenstructure of the matrix. You can even recover the angle from the **trace**: θ = arccos(trace( R)/2). It's a beautiful example of how linear algebra concepts connect: the same rotation can be understood through matrix multiplication, determinants, eigenvalues, or the trace, all giving consistent, complementary views of the same transformation.



