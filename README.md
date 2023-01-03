# Rusty Boids

This project develops a [Boids simulation](https://en.wikipedia.org/wiki/Boids) in Rust.  This is being primarily developed as an opportunity for me to learn Rust!

## Analysis

Each boid should have a position and velocity, both of which vary with time.  Call these variables $x_i(t)$ and $v_i(t)$.  The Boids algorithm describes several forces that act on the boids.  For now let's lump all of these forces together into a term $f_i(t)$.  From here on let's omit the subscript $i$ to keep notation neat.

To assemble the dynamical system from a physics perspective, we just need to combine the equations of position, velocity and force/accerlation.  First, position and velocity are related through $\dot{x}(t) = v(t)$.  Next, we can relate velocity and force using $f(t) = ma(t) = m\dot{v}(t)$.  Combining these two equations gives us the first-order dynamical system that we'll be working with.
$$
\frac{dv}{dt} = f(t)/m \\
\frac{dx}{dt} = v(t)
$$

### Nondimensionalization

Let 
1. $t = \tau t_c$,
2. $x(t) = \xi(\tau) x_c$, and
3. $v(t) = w(\tau) x_c/t_c$
where $x_c$ and $\tau_c$ are characteristic units of length and time, and $\xi$ and $\tau$ are dimensionless functions.  

To convert our dimensional equations to nondimensional equations we'll need to make some substitutions.  First note that taking the derivative of $\tau$ with respect to $t$ in the above definitions yields
$$
\frac{d\tau}{dt} = t_c^{-1}
$$

Now we can express $x(t)$ and $v(t)$ as dimensionless functions by taking the time derivatives of definitions 2 and 3.
$$
x(t) = \chi(\tau) x_c
    \implies \frac{dx(t)}{dt} = \frac{x_c}{t_c} \frac{d\chi(\tau)}{d\tau} \\
v(t) = w(\tau) x_c/t_c 
    \implies \frac{dv(t)}{dt} = \frac{x_c}{t_c}\frac{dw(\tau)}{d\tau}.
$$

Plugging all of this back into our dynamics equations finally yields
$$
\frac{dw(\tau)}{d\tau} = \frac{t_c}{mx_c}f(t) \\
\frac{d\chi(\tau)}{d\tau} = w(\tau)
$$