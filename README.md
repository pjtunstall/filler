# filler

- [Context](#context)
- [Versions](#versions)

## Context

This project is an implementation of the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) of an exercise called filler. The challenge is to create a bot that can compete against others in a game of placing variously shapes on a rectangular playing area. We're given several executable files: a "game engine" and four AI opponents. When run with the appropriate command-line arguments, the game engine will run the student bot together with one of the given bots. It will send random shapes (like generalized Tetris shapes) to each bot in turn along with the current state of the board. The bot must place this shape on the board in such a way that precisely one cell (character) of the new shape overlaps with one of the shapes it placed previously, thus increasing its territory. It mustn't overlap any cell of any of the opponent's territory.

Points are awarded for every shape placed. The instructions don't say whether shapes are worth equal points.

A small technicality, which I'll just mention here to clarify the terminology, is that the shape that a bot receives is actually sent embedded in a rectangle as in the following examples. The rectangle is referred to as a piece. The instructions offer the following examples of pieces.

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

To place a piece, a bot writes its coordinates (i.e., we must infer, the coordinates of its top left corner) to stdout, separated by a space and followed by a newline.

Eventually one of the players will run out of space and should then make an illegal move: "If your robot can't place anymore peaces\[sic\] he should still return a result (even if invalid), our robots for example return `0 0\\n`, when they can't place any more pieces." The instructions don't sepcify whether this forced invalid move has to be correctly formatted at least, or within the bounds of the board, although perhaps this is implicit in the audit question "Can you confirm that the project runs correctly?" If one player crashes or fails to send anything till the game engine imposes an unspecified timeout, they lose and the game ends there.

The challenge is to defeat three of the given robots on at least four out of five games. Bonus marks are to be had for defeating the most formidable opponent, terminator.

## Versions

This project is my attempt at the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) // [01Founders version](https://learn.01founders.co/intra/london/div-01/filler) of the exercise, which is the same as the [42 School version](https://github.com/VBrazhnik/Filler/blob/master/filler.en.pdf), apart from trivial differences: 42 School calls the board "plateau", while 01Edu calls it "Anfield", and different symbols are used for the territories of the two players and their latest moves. (I found the 42 School instructions worth reading too, though, as they're more detailed than those of 01Edu--see especially the longer example of gameplay in §V.4.3, p. 11--and randomly contain a list of the Seven Deadly Sins, complete with Biblical quotations to keep you on the right track!)

```
| Meaning              |  01Edu  | 42 School |
|----------------------|---------|-----------|
| Player 1             |    @    |     O     |
| Player 2             |    $    |     X     |
| Player 1 latest move |    a    |     o     |
| Player 2 latest move |    s    |     x     |
| Empty                |    .    |     .     |
| New piece            |    O    |     *     |
```

Regarding the 01Edu new-piece symbol, the shape cells of the three example pieces in the [pieces](https://github.com/01-edu/public/tree/master/subjects/filler#the-pieces) section are all denoted by '#', but those of the example piece in the [Usage](https://github.com/01-edu/public/tree/master/subjects/filler#usage) section by 'O' (uppercase letter after 'N'). At first, I guessed that the 'O' in the Usage example might be an accidental relic of an earlier version where Player 1's symbol was 'O', as in the 42 School instructions. But, on running the game engine, I see that 01Edu's current Linux game engine currently uses 'O', not '#', for its new-piece symbol.

## Usage

Open a terminal, clone this repository, and navigate into the root of the project:

```bash
git clone https://github.com/pjtunstall/filler
cd filler
```

Compile the executable file statically:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

The reason for static compilation is that the docker container provided has an old version of libc, which caused the dynamic linker to fail to load my binary otherwise. Then move the binary as follows:

```bash
mv target/x86_64-unknown-linux-musl/release/filler ../filler./docker_image/solution/
```

assuming the zip provided has been unzipped as `filler.` in the same folder as the Rust project.

Navigate into the docker_image folder, then build and run the docker container:

```bash
cd filler./docker_image
docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

## Questions

### Can pieces be rotated?

Apparently not. There's no way to express it.

### Negative coordinates

Sometimes it might be necessary for a player to send negative numbers as the coordinates of the piece (i.e. its top-left cell). Not all legitimate moves can be expressed otherwise, given the possibility of pieces such as

```
Piece 5 4:
.##..
.##..
..#..
...#.
```

Are negative coordinates accepted by the game engine though? At least it seems that the given bot terminator chooses invalid coordinates rather than negative ones, as can be seen by trying this random seed.

```
./linux_game_engine -f maps/map01 -p2 solution/filler -p1 linux_robots/terminator -s 1749393971253574634

```

I wonder if the audit requirement to change "the position of the players each time so that the student player can be the p1 and the p2" is to ensure that players have a roughly similar chance of getting stuck on the first move.

Revised opinion: negative coordinates are admitted. Swapping the labels of the two players in the example above and having one's own bot move output 4 -1 for the first move allows it to place

```
.....
.....
.....
.....
OOO..
OO...
```

on its initial cell, 4 3.

### Exit strategy

Should the bot exit after playing its final move? Not necessary.
