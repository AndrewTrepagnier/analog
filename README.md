# Linear Algebra Anaolog Clock


The goal of this project is to use linear algebra techniques in a creative way to manipulate the operating system’s (macOS) current time into an analog-style ASCII representation, using as few crates as possible.

No large language models, no lookup tables, no borrowing from other ASCII clock designs on GitHub — the idea was to build this in a pen-and-paper style, with particular emphasis on matrix operations and geometric transformations.

Time has always been an interesting concept to me, but the real motivation behind this project goes beyond visualizing it. I wanted to explore how linear algebra can drive creative software behavior, while also deepening my understanding of Rust’s crate architecture and standard library. For Python, that exploration happens through the Python Package Index. For Rust, it’s through Cargo crates — and understanding their structure gives a far more visceral understanding of how the language itself is designed.

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
