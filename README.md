# filler

- [Context](#context)
- [Versions](#versions)

## Context

This project is an implementation of the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) of an exercise called filler. The challenge is to create a bot that can compete against others in a game of placing variously shaped tiles on a rectangular playing area. We're given several executable files: a "game engine" and four AI opponents. When run with the appropriate command-line arguments, the game engine will run the student bot together with one of the given bots. It will send randomly shaped tiles (like generalized Tetris pieces) to each bot in turn along with the current state of the board. The bot must place this tile on the board in such a way that precisely one cell of the new piece overlaps one of the pieces it placed previously. It mustn't overlap any of the opponent's pieces.

Points are awarded for every tile placed. The instructions are unclear as to whether tiles are worth equal points.

Eventually one of the players will run out of space and should then make an illegal move: "If your robot can't place anymore peaces\[sic\] he should still return a result (even if invalid), our robots for example return 0 0\\n, when they can't place any more pieces." The instructions don't sepcify whether this forced invalid move has to be correctly formatted at least, or within the bounds of the board, although perhaps this is implicit in the audit question "Can you confirm that the project runs correctly?" If one player crashes or fails to send anything till the game engine imposes an unspecified timeout, they lose and the game ends there.

The challenge is to defeat three of the given robots on at least four out of five games. Bonus marks are to be had for defeating the most formidable opponent, terminator.

## Versions

This project is my attempt at the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) // [01Founders version](https://learn.01founders.co/intra/london/div-01/filler) of the exercise, which is the same as the [42 School version](https://github.com/VBrazhnik/Filler/blob/master/filler.en.pdf), apart from trivial differences: 42 School calls the board "plateau", while 01Edu calls it "Anfield", and different symbols are used for the territories of the two players and their latest moves:

```
| Meaning              | 01Edu | 42 School |
|----------------------|-------|-----------|
| Player 1             |   @   |     O     |
| Player 2             |   $   |     X     |
| Player 1 latest move |   a   |     o     |
| Player 2 latest move |   s   |     x     |
| Empty                |   .   |     .     |
| New piece            |   *   |     *     |
```

I found their instructions worth reading too, though, as they're more detailed than ours--see especially the longer example of gameplay in §V.4.3, p. 11--and randomly contain a list of the Seven Deadly Sins, complete with Biblical quotations to keep you on the right track!
