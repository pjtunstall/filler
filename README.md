# filler

- [Context](#context)
- [Versions](#versions)
- [Usage](#usage)
- [FAQ](#faq)
  - [Can pieces be rotated?](#can-pieces-be-rotated)
  - [Should your bot exit after playing its final move?](#should-your-bot-exit-after-playing-its-final-move)
  - [Can you send negative coordinates?](#can-you-send-negative-coordinates)
  - [Can pieces extend off the bottom or right of the grid?](#can-pieces-extend-off-the-bottom-or-right-of-the-grid)
- [Notes](#notes)

## Context

This project is an implementation of the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) of an exercise called filler.

The challenge is to create a bot (program) that can defeat another bot at a game played on a rectangular board. We're given several executable files: a so-called "game engine" and four opponents. When run, the game engine will launch the two bots specified by command-line arguments. It will send random shapes (like generalized Tetris shapes) to each bot in turn along with the current state of play. The bot must place this shape on the board in such a way that precisely one cell (character) of the new shape overlaps with one of the shapes it placed previously, thus increasing its territory. It mustn't overlap any cell of the opponents territory.

Points are awarded for every shape placed. The instructions don't say whether shapes are worth equal points.

A small technicality, which I'll just mention here to clarify the terminology, is that the shape that a bot receives is actually sent embedded in a rectangle as in the following examples. The surrounding rectangle is referred to as a piece. The instructions offer the following examples of pieces.[^1]

```
Piece 2 2:
.O
O.

Piece 5 4:
.OO..
.OO..
..O..
...O.

Piece 6 3:
.OO...
OOO...
O..O..
```

To place a piece, a bot writes its coordinates (i.e. the coordinates of its top left corner) to stdout, separated by a space and followed by a newline.[^2]

To be valid, a move must not extend the player's territory outside the edges of the board.

Eventually one of the players will run out of space and should then make an illegal move: "If your robot can't place anymore peaces\[sic\] he should still return a result (even if invalid), our robots for example return `0 0\n`, when they can't place any more pieces." The instructions don't say whether this forced invalid move has to be correctly formatted, although this might be implicit in the audit question "Can you confirm that the project runs correctly?" If one player crashes or fails to send anything till the game engine imposes a timeout (by default 10s, but configurable), they lose and the game ends there.

The challenge is to defeat three of the given robots on at least four out of five games. Bonus marks are to be had for defeating the most formidable opponent, terminator.

## Versions

This project is my attempt at the [01Edu version](https://github.com/01-edu/public/tree/master/subjects/filler) / [01Founders version](https://learn.01founders.co/intra/london/div-01/filler) of the exercise, which is similar to the 42 School Instructions [English](https://github.com/VBrazhnik/Filler/blob/master/filler.en.pdf) / [French](https://github.com/ivankozlovcodes/filler/blob/master/resources/filler.pdf).

One potential difference is that their version of the game is said to stop as soon as one player can't make a legal move, whereas ours continues alowing the other player to place pieces (and hence collect points) as long as they can after that.

I say "potential" because there are some contradictions over this rule. The 42 School instructions say, "The game stops at the first error: either when a game piece cannot be placed anymore or it has been wrongly placed." ("La partie s'arrête à la première erreur: dès qu'une pièce ne peut plus posée ou a été mal posée.") On the other hand, [Ivan Kozlov](https://github.com/ivankozlovcodes/filler/blob/master/resources/usage.gif) at 42 School Silicon Valley in 2018 shows the game continuing in an animated example of his visualizer in action, and [Jani Mäkelä](https://github.com/dal-yth/Filler) at Hive-Helsinki in 2020 remarks, "This repository has both the new (vm2) and old (vm) game masters [i.e. game engines], it is recommended to use the old one (filler_vm.rb) since the new one does not work well with the champions provided. Some of the champions refuse to place any pieces with the new executable and it also stops the game before the winning[^3] player can attempt to fill out the remaining field."[^4]

There are also some trivial differences: 42 School calls the board "plateau" (in both English and French), while 01Edu calls it "Anfield", and different symbols are used for the territories of the two players and their latest moves.

I'd say the 42 School instructions still worth reading even if your objective is only to understand the 01Edu version. They're more detailed than those of 01Edu--see especially the longer example of gameplay in §V.4.3.

```
| Meaning              |  01Edu  | 42 School |
|----------------------|---------|-----------|
| Player 1             |    @    |     O     |
| Player 2             |    $    |     X     |
| Player 1 latest move |    a    |     o     |
| Player 2 latest move |    s    |     x     |
| Empty                |    .    |     .     |
| New piece            |    O    |     *     |
| Board                | Anfield |  plateau  |
```

## Usage

Open a terminal, clone this repository, and navigate into the root of the project:

```sh
git clone https://github.com/pjtunstall/filler
cd filler
```

Download the docker_image folder as a zip file [here](https://assets.01-edu.org/filler/filler.zip) from the 01Edu public repo. To suppress the warning `JSONArgsRecommended: JSON arguments recommended for ENTRYPOINT to prevent unintended behavior related to OS signals (line 11)` that would otherwise appear when you build the container, change the final line of the Dockerfile from

```Dockerfile
ENTRYPOINT /bin/bash
```

to

```Dockerfile
CMD ["/bin/bash"]
```

Compile the binaries for my bot and visualizer:

```sh
cargo build --release
```

Move or copy them to `docker_file`, noting that the unzipped folder would have been called `filler` but needs some distinguishing mark to make it different from the project folder. On Linux, at least, a final `.` was supplied automatically.

```sh
cp target/release/maximilian ../filler./docker_image/solution/
cp target/release/visualizer ../filler./docker_image/
```

Optionally, copy the launch script there too:

```sh
cp launch.sh filler./docker_image/
```

This will let you run the container with `./launch.sh`, as a convenience, instead of having to type the elaborate run command, `docker run` etc., from the code block that follows.

Navigate into the `docker_image` folder, then build and run the docker container:

```sh
cd filler./docker_image
docker build -t filler .
docker run -v "$(pwd)/solution":/filler./solution -it filler
```

You should now be in a shell session inside the container. To run a game, choose a map and two opponents, e.g. to pit my bot against their terminator:

```sh
./linux_game_engine -f maps/map01 -p1 solution/maximilian -p2 linux_robots/terminator
```

To run with the visualizer, exit docker and, on a host machine terminal, enter:

```sh
./linux_game_engine -f maps/map01 -p1 solution/maximilian -p2 linux_robots/terminator | ./visualizer
```

Optionally, you can specify a scale (size) for the visualizer window and/or a duration to wait after parsing and drawing each move.

```
Usage: ... | ./visualizer [-s|--scale SCALE] [-d|--duration DURATION]
Defaults:
  SCALE      = 20
  DURATION   = 16 (milliseconds)

Examples:
  program -s 30 -d 50
  program --scale 25
  program --duration 75
```

You can exit the game at any time with Ctrl+C, or press escape to exit the visualizer.

## FAQ

### Can pieces be rotated?

Apparently not. There's no way to express it to the game engine.

### Should your bot exit after playing its final move?

No. Indeed, it's perfectly possible to get stuck while your opponent continues to play and yet you win on points because your opponent didn't have enough space.

### Can you send negative coordinates?

Yes. Sometimes it might be necessary for a player to send negative numbers as the coordinates of the piece (i.e. its top-left cell). Not all legitimate moves can be expressed otherwise. At first, I wasn't sure whether negative coordinates were accepted by the game engine. The instructions are silent on this point. It seems that the given bot terminator chooses invalid coordinates rather than negative ones, as can be seen by launching the game with this random seed:

```sh
./linux_game_engine -f maps/map01 -p2 solution/maximilian -p1 linux_robots/terminator -s 1749393971253574634

```

I did wonder if the audit requirement to change "the position of the players each time so that the student player can be the p1 and the p2" was meant to ensure that players have a roughly similar chance of getting stuck on the first move.

And yet, negative coordinates are admitted! Swapping the labels of the two players in the example above and having your own bot move output 4 -1 for the first move allows it to place

```
.....
.....
.....
.....
OOO..
OO...
```

on its initial cell, 4 3.

### Can pieces extend off the bottom or right of the grid?

Can empty cells of pieces exceed the bottom or right edges of the board? Yet to be determined, but I'm guessing empty cells can go anywhere as long as we follow the rule: "The shape of robots territory must not exceed the area of the board."

## Notes

[^1]: The 'O' in these three examples from the [The pieces](https://github.com/01-edu/public/tree/master/subjects/filler#the-pieces) section is actually a '#', but this must be a typo or a relic from an earlier version, so I've corrected it here. The example in the [Usage](https://github.com/01-edu/public/tree/master/subjects/filler#usage) section has 'O' (uppercase letter after 'N'), as does the current game engine.
[^2]: The "coordinates" of a piece are nowhere definied explicitly, as far as I can see, but can be inferred from the fact that `7 2\n` is a legitimate way to place `.OO.` in the example of the [Usage](https://github.com/01-edu/public/tree/master/subjects/filler#usage) section, given that the player's territory so far consists of just one cell, `9 2`.
[^3]: When one player gets stuck, the other doesn't necessarily win. The first player to get stuck might still have more more points at the end.
[^4]: The latter possibility seems more in keeping with the variety of strategies that Jani considers an interesting quality of the game: "... you can approach it in so many different ways. Perhaps your algorithm attempts to seal off half of the map and survive until the bitter end, perhaps you try to box your opponent in so they can't place any more pieces or maybe you try to breach into your opponents area and take over the space they were saving for late game."
