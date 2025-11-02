# Linear Algebra Anaolog Clock


The goal of this project is to use linear algebra techniques in a creative way to manipulate the operating system’s (macOS) current time into an analog-style ASCII representation, using as few crates as possible.

No large language models, no lookup tables, no borrowing from other ASCII clock designs on GitHub — the idea was to build this in a pen-and-paper style, with particular emphasis on matrix/vector transformations using dot product tricks.

Time has always been an interesting concept to me, but the real motivation behind this project goes beyond visualizing it. I wanted to explore how linear algebra can drive a program's functionality, while also deepening my understanding of Rust’s crate architecture and standard library. For Python, that exploration happens through the Python Package Index. For Rust, it’s through Cargo crates — and understanding their structure gives a far more visceral understanding of how the language itself is designed.

Because these are my focus areas, the clock itself is not very asthetically pleasing (as you can see below); however, the logic should be technically sound.

Analog Clock Face:
```
                              12 12                               
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
                               6  6 

```
[ Q0 | Q1 ]

[ Q2 | Q3 ]






```
                              12 12                               
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
  +                           Origin (X,Y)                   +   
 9                                                              3 
 9                              +  >  >  >    >  >   >   >   >  3 
  +                                                         +      (3 o'clock coordinate (w1, w2)
    +                                                     +     
      +                                                 +       
        +                                             +         
          +                                         +          Reference vector becomes origin-3'oclock vector when it sees the time is between 3-6. 
            +                                     +            From there, what is the ratio of minutes to 60? for example, 34 minutes / 60 minutes = 56% completed with that hour  
              +                                 +              between 3 and 4. Find the overall angle between reference vector and 4 position, then 56% of that is the angle that 
                +                             +                our hour hand landing point will have between that and the reference line. This can be done with a dot product, since 
                  +                         +                  we our origin coordinate, we know 3's coordinate, we know 4's coordinate (will be 5 dots diagonal from 3) and we can  
                    +                     +                    calculate the angle it should have from reference vector. This should be a function that takes time parameters, seconds 
                      +                 +                      and minutes as arguments. 
                        +             +                         
                          +         +                           
                            +     +                             
                               6  6 

```
