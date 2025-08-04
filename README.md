# ❗ I suggest you DON'T use this. ❗
This project was an **experiment** for a game demo project of mine. I used this game engine to try and learn some of the principles behind the ECS model by attempting to make a "hybrid engine" between OOP/ECS.

I decided to fold the game idea as it was clear it wasn't very fun.

Overall I'd call it a success! Mainly because I now have a better understanding of ECS as a concept and how I can use it. Previously I struggled to wrap my head around it.

## Why you shouldn't use this
I'm making this as a personal learning tool, and it was extracted out of a much larger project - meaning:
- There's no documentation
- The code wasn't written with the intent to be used
- The code wasn't written with the intent to be legible
- It might not compile out of the box, and may require heavy tweaking to actually get things done
- It is missing a vast majority of the features that you would want from a game engine

And also, from personal experience:
- Merging OOP and ECS doesn't bring the strengths of both systems together, it brings the *weaknesses* together - the engine is horrible to work in
- It runs poorly. I was developing on a RTX 2070 + AMD Ryzen 7 3700X w/ 16GB of RAM on a 1440p display. I got ~200fps and frame time was *unstable*.

## If you decide you still want to use this, here's a brief overview
`Scene`s hold objects.
Objects can implement "behaviours" of the engine, which are then run on every frame of the `Scene`.

So if you want `Player` to be able to draw to the screen - then `Player` needs to implement `Drawable`. Then, when it is placed in the scene, you need to spawn it in as "drawable" (there's a macro to help).

Good luck!
