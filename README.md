# filler

- [Context](#context)
- [Versions](#versions)

## Context

This project is an implementation of the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) of an exercise called filler. The challenge is to create a bot that can compete against others in a game of placing variously shapes on a rectangular playing area. We're given several executable files: a "game engine" and four AI opponents. When run with the appropriate command-line arguments, the game engine will run the student bot together with one of the given bots. It will send random shapes (like generalized Tetris shapes) to each bot in turn along with the current state of the board. The bot must place this shape on the board in such a way that precisely one cell (character) of the new shape overlaps with one of the shapes it placed previously, thus increasing its territory. It mustn't overlap any cell of any of the opponent's territory.

Points are awarded for every shape placed. The instructions don't say whether shapes are worth equal points.

A small technicality, which I'll just mention here to clarify the terminology, is that the shape that a bot receives is actually sent embedded in a rectangle as in the following examples. The rectangle is referred to as a piece.

```
Piece 2 2:
.#
#.

Piece 5 4:
.##..
.##..
..#..
...#.

Piece 6 3:
.##...
###...
#..#..
```

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

## Questions

### Can pieces be rotated?

I'm guessing not.

### Negative coordinates

Sometimes it might be necessary for a player to send negative numbers as the coordinates of the piece (i.e. its top-left cell). Not all legitimate moves can be expressed otherwise, given the possibility of pieces such as

```
Piece 5 4:
.##..
.##..
..#..
...#.
```

Are negative coordinates accepted by the game engine though?

### Inconsistent symbols

The shape cells of the example pieces in the [pieces](https://github.com/01-edu/public/tree/master/subjects/filler#the-pieces) section are all denoted by '#', but those of the example piece in the [Usage](https://github.com/01-edu/public/tree/master/subjects/filler#usage) section by 'O' (uppercase letter after 'N'). I'm guessing this is an accidental relic of an earlier version where Player 1's symbol was 'O', as in the 42 School instructions?

## Plan

I could keep a list of `piece::Cell`s in own and opponent's territory, then iterate through them rather than checking all potential coordinates to place a piece. That might be faster at the start. Or it might be slower than iterating over all the coordinates, which are, after all, only numbers, not elements stored on the stack.

Do I really need piece::Cell as well as anfield::Node?

Encapsulate access to grids as 1d arrays so there are fewer places for the calculation to go wrong.
