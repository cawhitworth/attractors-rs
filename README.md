# attractors-rs

![Clifford with coefficients: a: -1.8366614713744718, b: 1.7629891558685542, c: 0.19188594857654007, d: 1.174854886118256](https://raw.githubusercontent.com/cawhitworth/attractors-rs/main/examples/example_1.png)

Rendering attractors in Rust. A port of my [C++ Attractors](https://github.com/cawhitworth/Attractors) project.

As well as rendering the images using custom gradient palettes and configurable
gamma and exposure, the code attempts to find aesthetically interesting sets of
coefficients automatically.

Initial inspiration came from [this](http://paulbourke.net/fractals/clifford/)
popping up on Hacker News.

## Attractor functions?

Iterative or derivative functions that tend towards a particular set. Sometimes
this is a single value, or a cycle of a handful of values. Sometimes, it's a
large set of values that when plotted, draw a pretty pattern. This project
tries to find and draw the latter.

## Finding interesting patterns

The code essentially looks for sets of coefficients that, when the attractor
function is iterated, produce a large number of different values quickly. The
way it does this probably isn't the most efficient.

This process repeats with randomly chosen coefficients until an 'interesting'
set is found.

First, it runs 10,000 iterations of the pure function, keeping track of the
minimum and maximum x and y values - this firstly gives us the bounds for the
function so we can scale when rendering, and secondly, if any of the bounds is
still zero, we can take a reasonable guess at this not being an interesting set
of coefficients.

Next, it runs another 10,000 iterations of an exposure pass (see below) on a
640x512 bitmap. If the maximum exposure recorded during this pass is greater
than 10 - i.e., if any single pixel in the bitmap is visited more than 10
times - we consider this set of coefficients uninteresting.

## Rendering

This is a two-stage process. First, the image is *exposed*, and then
*developed*.

### Exposure

The function is repeatedly evaluated; after each iteration, the resulting x
and y values are mapped back onto a bitmap (using the bounds we calculated
in the evaluation stage). The value in the corresponding pixel in the bitmap
is incremented by one, and the maximum value seen in any pixel is tracked -
the `max_exposure` value in the code.

### Developing

Once the image is exposed, we have a grid of values between `0` and
`max_exposure`. The develop stage maps these values onto the range from 0-1
using the `max_exposure` and a gamma correction (i.e., `p -> p ^ 1/gamma`). The
result is clamped to the range 0-1, so using a value lower than `max_exposure`
will increase the effective exposure of the image and the expense of blowing
out the brighter end.

Once the adjusted exposure value has been calculated, it is mapped onto a
colour gradient defined by a set of values from 0-1 and corresponding RGB
values; the colours are linearly interpolated appropriately.

## Rust porting notes

I've tried to keep this as close to a direct port of my C++ as possible -
function and variable names should be broadly the same, modulo language style
choices, and the design of the solution (passing functions around and partially
binding them) remains the same.

### bind_1

Because of Rust's requirements around lifetimes, it turns out passing functions
and closures can get considerably more complex than in other languages.

```rust
pub fn bind_1<'a, T, U, V, F>(function: &'a F, u: &'a U) -> impl Fn(&T) -> V + 'a 
    where F: Fn(&T, &U) -> V {
    move |t| function(t,u)
}
```

`bind_1` takes a function which, in turn takes two parameters (of type T and
U respectively) and returns a value of type V, and a value of type U. It returns
a function which takes a parameter of type T and returns the result of calling the
first function with that parameter and the U-value passed in.

Because we are borrowing both the original function and the U-value passed in and
creating a closure over both of them, we need to specify that the returned function
(the closure) has (at least) the same lifetime - neither the function nor the bound
value may be dropped before the returned function.
