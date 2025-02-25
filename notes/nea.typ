#set par(justify: true)
#show link: underline
#set page(
  numbering: "1",
  margin: 2cm,
  paper: "us-letter",
) 
#set text(
  hyphenate: false,
  font: "Jetbrains Mono"
)
#set heading(numbering: "1.", offset: 0)
#set text(12pt)
#set enum(numbering: "1.1", full: true)
#set list(marker: ([•], [‣],[--]))
#set math.mat(delim: "[");
#set math.vec(delim: "[");


#page(numbering: none, [
  #v(2fr)
  #align(center, [
    #text(23pt, weight: 700, [Multiplayer Grid Based Dexterity Training Game])
    #v(0.1fr)
    #text(23pt, weight: 500, [NEA: Georgiy Tnimov])
    #v(1.1fr)
  ])
  #v(2fr)
])


== Abstract


== Client

=== Client Synopsys
The Client is Alexander Tahiri, a software developer at Studio Squared and the developer of Tapp, a game based on a 4x4 grid, which consists of 12 inactive tiles, and 4 active tiles. Players use the mouse cursor to click on an active tile, which then deactivates that tile and actives a new, currently non-active tile.the objective of Tapp is to achieve as high a score as possible, without making any mistakes. The Client requires a derivative of this game, which tests simultaneous dexterity of both hands, additionally The Client wants to incorporate a competitive aspect to the game, which consists of a leaderboard section, allowing players to see their position whithin the rankings and a Tetris-99-esque game mechanic, where players compete to either achieve the highest score, or last the longest in a mass multiplayer format. The Client has specifically asked for the Catppuccin colour scheme to be used, The Client has sufficient computing power to host both the client,server and database, which will be provided free of charge.


=== Interview Notes
*all interview notes are paraphrased* \
"for doubletapp, what features are most important to you?" \
AT: my main requirement is that the new game tests both hands simultaneously, and has replayability. features such as users and leaderboards, along with a competitive aspect would be awesome \
"how many users do you expect to scale to?" \
AT: I am estimating up to 50 concurrent users, and aim for small latencies \
"any specific UI/GUI choices, and what platform should doubletapp support" \
AT: doubletapp should be a website, like the original tapp, and it should use the catppuccin colour scheme. \
"any specific technologies you would like implemented?" \
AT: I am a fan of Svelte, and would like to use rust as the backend due to its fast speeds and growing technology base, Tapp doesn't have a database but SQL would be acceptable. \
"doubletapp might have a cheating proble, would you like an anticheat?" \
AT: an anticheat would be desirable, due to Svelte being unobfuscated a server side anticheat might be best \



== Research
https://rust-lang.github.io/async-book/
//TBD
// xoshiro, websockets, tailwindcss,async

== Prototyping
A rudimentary prototype has been made, which tested out multiple different input methods for simultaneous inputs, which has finalized in a "cursor"-based system, where you have two cursors controlled by Wasd-like movement, with each set of controls representing their respective cursor, additionally it has been decided that both cursors need to be on individual Tiles, to prevent copying movements on each hand. this prototype also implemented server-side move verification, making it more difficult to cheat. Finally, the UI design of the prototype will be used in later iterations of the project.
//image of doubletapp

== Documented Design

=== Algorithms

==== Xoshiro256+

==== Manhattan Distance

==== MergeSort

==== Std Dev + Variance

=== Data Structures

==== Circular Queue
A queue is a data structure following the FIFO (first in first out) principle, where you use a sized array, along with variables to store the capacity, front & back of the array, when a file is queued, the file is put onto the index of the back of the array, and then the back index is added to % capacity unless the back becomes equal to the front, in which the queue returns an error instead, this allows for a non resizable array, which allows a set amount of elements to be queued, but not more than the size of the array, allowing for efficient memory management

==== HashMap


==== Option/Result Types
=== Function Map



=== Class Diagrams


=== Database Design


=== Mockups & etc..

== Objectives

=== User Interface
+ user can interact with the grid
  + user can move both cursors using keyboard on the grid
  + user can "submit" moves using a keybind
  + user can reset game (in single player) via a keybind
+ user can change gamemode (singleplayer,multiplayer) on the main page
  + user can change grid size (4x4,5x5,6x6) in singleplayer
  + in singleplayer, user can change time limit (30,45,60)
+ user can access settings
  + user can modify keybinds for each action in the game
  + user can change DAS 
  + user can change ARR 
  + user can log out of account
  + user can reset all keybinds to a sane default
+ user can play the game
  + on game start, user sees cursors are positioned on opposing sides of the board
  + on game start, user sees the starting active tiles
  + user can view current game score
  + in singleplayer, user sees time remaining
  + in multiplayer, user can see time remaining for current quota, players remaining and current score
  + user is notified of their position in the multiplayer game
  + user can "submit" their move
    + user can interactively see if the move was valid via a colour interaction which flashes green or red depending on if the move was valid, a valid move is when the two cursors are on two active grid tiles within the grid boundary and they are distinct active tiles
    + on successfull submit, user sees two new tiles become active, which were previously inactive and are not on current cursor location
  + cursors are rendered via two different colours, with the two cursors being visually distinct but symmetrically consistent 
+ user can see statistics post singleplayer game end
  + user views their score
  + user views if their score was validated by the server
  + user views their leaderboard position
  + user can copy their game statistics to the clipboard for sharing
  + if user is logged in and not marked as a cheater, user can view their game in the statistics page
  + user has the option to start a new game from the results menu
+ user can view leaderboard
  + user can view leaderboards, in a paginated format 
+ user can play the multiplayer gamemode
  + user can see the other players movements on other grids in the game
  + user can see their remaining score quota for each 5 second interval period
  + after a user has been eliminated by not reaching the quota,the user can view their position in the game
+ user can log in to the application
  + user can login or signup depending on their requirements
  + user is shown error codes depending on if account already exists or their login details are incorrect
  
=== Server Side
+ User CRUD
  + simple user authentication
    + simple verification of authenticity, i.e password hashing & username uniqueness check
+ Database Schema
  + contains user table
  + contains game table, which stores all real authenticated games (not including moves)
  + contains linked user statistics table
+ Game Verification
  + server verifies all moves are valid
  + server verifies that move positioning is within human bounds, i.e ratio of "optimal moves" and timing distribution
  + server verifies that game was submitted within the time limit (with a grace period)
+ Multiplayer implementation
  + server can communicate actions bidirectionally with client
  + each move is verified by the server
  + low latency communication between server and client
  + client can distinguish between types of messages recieved
