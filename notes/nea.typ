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

== Problem Definition
I plan to develop a game, which tests the dexterity of both hands, simultaneously. I believe its important that people can maintain their dexterity of both hands, and this game will help them do that. I also believe the game will be fun, and will be a good way to pass time. adding a competitive and multiplayer aspect to the game will also help with this. 

I plan to develop this game using Rust and Svelte, as well as a websocket server, which will be used to communicate between the client and server.

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


=== Similar Solutions
There are a few similar products on the market that test dexterity in various ways. Understanding these existing solutions helps position DoubleTapp in the competitive landscape and justify its development.

==== Tetris
Tetris is one of the most recognized dexterity-based puzzle games worldwide. While it effectively tests hand-eye coordination and spatial reasoning, it differs from DoubleTapp in several key ways:

- Tetris focuses primarily on single-hand dexterity, with players typically using one hand for directional controls and the other for occasional rotation/drop buttons
- It has a significant learning curve with complex strategies around piece placement and line clearing, i.e T-spins, Wall Kicks
- Players focus more on strategic planning of where to place pieces rather than pure dexterity training
- The modern competitive versions of Tetris (like Tetris 99) do incorporate multiplayer aspects, but interaction between players is indirect through "garbage lines"

Tetris has multiple useful features which I will be taking inspiration from, particularly Delayed Auto Shift (DAS)@DAS-Tetris, which allows for precise control of pieces, this allows for people to have more accurate control over their piece placement and allows for timing optimization
==== Tapp
Tapp, developed by Alexander Tahiri at Studio Squared, is the direct predecessor to DoubleTapp and shares the most similarities:

- Uses a grid-based interface (4x4) with active and inactive tiles
- Tests dexterity through rapid target acquisition
- Focuses on score maximization without mistakes
- Simple, accessible gameplay with minimal learning curve

However, Tapp is limited to single-hand dexterity training, using only mouse input. It lacks the simultaneous dual-hand coordination that DoubleTapp aims to develop. Additionally, Tapp has no built-in multiplayer functionality or competitive leaderboard system.

==== Other Dexterity Training Applications
Various other applications exist for dexterity training, including:

- Typing games that test two-handed coordination but in a highly structured, predictable pattern (monkeytype, nitrotype)
- Rhythm games (like Dance Dance Revolution or osu!) that test reaction time and coordination but typically focus on timing rather than spatial navigation
- Aim trainers (for FPS games) that focus exclusively on mouse precision, although sometimes incorporate simultaneous dexterity, i.e counterstrafing, bhopping, edgebugging

=== Multiplayer
for implementing multiplayer, there are multiple solutions that work, i.e unidirectional HTTP requests, custom UDP handling, and websockets

#table(
  columns: (1fr, 2fr, 2fr),
  inset: 12pt,
  align: (center, left, left),
  stroke: 0.7pt,
  fill: (_, row) => if row == 0 { luma(240) } else { none },
  [*Method*], [*Pros*], [*Cons*],
  
  [HTTP @HTTP-Protocol] , 
  [
    - Simple implementation
    - Reasonably performant
    - Easily Debuggable
    - widely supported
  ], 
  [
    - Slow with many simultaneous users
    - Requires entire connection sequence for each request
    - relatively high latency
    - not designed for bidirectional communication
  ],
  
  [UDP @UDP-Protocol], 
  [
    - Very performant
    - allows for low level optimisations
    - minimal overhead
  ], 
  [
    - susceptible to packet loss, and is not guaranteed to have data parity (important for doubletapp)
    - complex to implement, and difficult to interconnect with existing libraries without significant performance declines
    - often blocked by firewalls
    - no ordering guarantees
  ],
  
  [Websockets @WebSockets-Guide], 
  [
    - allows for fast and safe data transmission
    - relatively complex to implement, as need to handle assignment of websockets to individual games
    - compatible with existing web server libraries
    - fully duplex, no need to reestablish connection sequence each request
  ], 
  [
    - websockets don't recover when connections are terminated
    - some networks block the websocket protocol, limiting accessibility
    - high memory usage per connection compared to UDP/HTTP
  ]
)

I have decided to use websockets, as they are a reasonable balance of complexity, performance, and ease of implementation, while still providing a high degree of reliability and safety. 
=== PRNG's (Pseudorandom Number Generators)

after considering many PRNG's (pseudorandomnumber generators), for example ARC4 , seedrandom, ChaCha20, and discounting them due to performance issues / hardware dependent randomization, I decided on using the Xoshiro/Xoroshiro family of algorithms, which are based on the Linear Congruential Generators, which are a (now-obsolete) family of PRNG's, which use a linear multiplication combined with modulus operations, to create quite large non-repeating sequences, although quite slow and needing very large state. xoshiro generators use a much smaller state (between 128-512) bits, while still maintaining a large periodicity,


#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: (left, left, left),
  stroke: 0.7pt,
  [*PRNG Algorithm*], [*Pros*], [*Cons*],
  
  [ARC4 (Alleged RC4)],
  [
  - Simple implementation
  - Fast for small applications
  - Variable key size
  ],
  [
  - Cryptographically broken
  - Biased output in early stream
  - Vulnerable to related-key attacks
  ],
  
  [Seedrandom.js],
  [
  - Browser-friendly
  - Multiple algorithm options
  - Good for web applications
  ],
  [
  - JavaScript performance limitations
  - Depends on implementation quality
  - Not cryptographically secure by default
  ],
  
  [ChaCha20],
  [
  - Cryptographically secure
  - Excellent statistical properties
  - Fast in software (no large tables)
  - Parallelizable
  ],
  [
  - Complex implementation
  - Overkill for non-security applications
  - Higher computational cost
  ],
  
  [Xorshift],
  [
  - Extremely fast
  - Simple implementation
  - Good statistical quality
  ],
  [
  - Not cryptographically secure
  - Simpler variants have known weaknesses
  - Some states can lead to poor quality
  ],
  
  [Linear Congruential Generator (LCG)],
  [
  - Simplest implementation
  - Very fast
  - Small state
  ],
  [
  - Poor statistical quality
  - Short period for 32-bit implementations
  - Predictable patterns
  ],
  
  [Mersenne Twister],
  [
  - Very long period
  - Good statistical properties
  - Industry standard in many fields
  ],
  [
  - Large state (2.5KB)
  - Not cryptographically secure
  - Slow initialization
  ],
  
  [Xoshiro256+/++],
  [
  - Excellent speed
  - Great statistical properties
  - Small state (256 bits)
  - Fast initialization
  ],
  [
  - Not cryptographically secure
  - Newer algorithm (less scrutiny)
  - Some variants have issues with specific bits
  ],
  
  [PCG (Permuted Congruential Generator)],
  [
  - Excellent statistical properties
  - Small state
  - Good performance
  - Multiple variants available
  ],
  [
  - More complex than basic PRNGs
  - Not cryptographically secure
  - Relatively new
  ]
)

#v(1cm) // Add vertical space between tables

// Second table: PRNG Metrics
#table(
  columns: (auto, auto, auto, auto, auto),
  inset: 10pt,
  align: (left, center, center, center, center),
  stroke: 0.7pt,
  [*PRNG Algorithm*], [*Estimated Time*], [*Cycle Length*], [*State Size*], [*Performance*],
  
  [ARC4],
  [Medium],
  [~$10^{100}$],
  [~256 bits],
  [Moderate],
  
  [seedrandom.js],
  [Medium],
  [(multiple selectable algorithms)],
  [Varies by algorithm],
  [Moderate (JS limited)],
  
  [ChaCha20],
  [High],
  [$2^{256}$],
  [384 bits],
  [High for crypto],
  
  [Xorshift],
  [Very Low],
  [$2^{128} - 1$],
  [128-256 bits],
  [Very High],
  
  [Linear Congruential Generator (LCG)],
  [Extremely Low],
  [Up to $2^{32}$ (typical)],
  [32-64 bits],
  [Extremely High],
  
  [Mersenne Twister],
  [Medium],
  [$2^{19937}-1$ (MT19937)],
  [2.5 KB (19937 bits)],
  [Moderate],
  
  [Xoshiro256+/++],
  [Very Low],
  [$2^{256}-1$],
  [256 bits],
  [Very High],
  
  [PCG (Permuted Congruential Generator)],
  [Low],
  [$2^{128}$ or more],
  [64-128 bits],
  [High]
)

after testing, xoshiro256+ has provided the best results, in terms of speed and simplicity of implementation, while still providing a high degree of randomness, and a large cycle length, which is important for a game such as DoubleTapp, where we want to ensure that the game is fair and that the same seed will not be repeated for a long time.
== Prototyping
A rudimentary prototype has been made, which tested out multiple different input methods for simultaneous inputs, which has finalized in a "cursor"-based system, where you have two cursors controlled by Wasd-like movement, with each set of controls representing their respective cursor, additionally it has been decided that both cursors need to be on individual Tiles, to prevent copying movements on each hand. this prototype also implemented server-side move verification, making it more difficult to cheat. Finally, the UI design of the prototype will be used in later iterations of the project.
//image of doubletapp

== Documented Design

=== Algorithms

==== Xoshiro256+

```rust
        // output is generated before the "next" cycle
        let result = self.seed[0].wrapping_add(self.seed[3]);
        // shifting prevents guessing from linearity
        let t = self.seed[1] << 17;
        // these 4 xor operations simulate a matrix transformation
        self.seed[2] ^= self.seed[0];
        self.seed[3] ^= self.seed[1];
        self.seed[1] ^= self.seed[2];
        self.seed[0] ^= self.seed[3];
        // last xor is just a xor
        self.seed[2] ^= t;
        // the rotation ensures that all bits in the seed eventually interact, allowing for much higher periodicity (cycles before you get an identical number, which in the case of xoshiro256+ is 2^256 - 1)
        self.seed[3] = Xoshiro256plus::rol64(self.seed[3], 45);
        // gets the first 53 bits of the result, as only the first 53 bits are guaranteed to be unpredictable for xoshiro256+, for the other variations i.e ++,*,** they are optimized for all the bits to be randomized, but as xoshiro256+ is optimized for floating points, which we require
        (result >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
```
==== Sigmoid Function
the sigmoid function is a function, that maps any real input onto a S shaped curve, which is bound between values, in my case i am bounding the output of the Xoshiro256+ float to be between 0..11, which allows me to easily use it to generate the "next" state of the game, allowing for a more natural distribution of numbers, as well as a more consistent distribution of numbers, which allows for a more consistent game experience.

```rust
// simple function, but incredibly useful
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

```
==== Djikstras Algorithm

==== MergeSort
mergesort is a sorting algorithm, which works by the divide and conquer principle, where it breaks down the array into smaller and smaller arrays, till it gets to arrays of length 2, which it then subsequently sorts from the ground up, returning a sorted array in O(nlog(n)) time complexity & O(n) space complexity

```rust
fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);
    
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_idx = 0;
    let mut right_idx = 0;
    
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            result.push(left[left_idx].clone());
            left_idx += 1;
        } else {
            result.push(right[right_idx].clone());
            right_idx += 1;
        }
    }
    
    result.extend_from_slice(&left[left_idx..]);
    result.extend_from_slice(&right[right_idx..]);
    
    result
}

```

==== Std Dev + Variance


==== Delayed Auto Shift
Delayed auto shift (DAS for short) is a technique implemented in tetris, where you wait for a period of time before starting to move the pieces, while the key is being held down, bypassing the operating systems repeat rate. This is useful for optimizing movements in games similar to DoubleTapp, or tetris, people can customize their DAS and their ARR(auto repeat rate) to be optimal for their own reaction time, so if they need to move a piece they can move it to the corners very quickly, but only after X time has passed, instead of the OS default of ~1 second for delay and ~100ms per repeat, in my algorithm I used the provided javascript api's of setTimeout and setInterval, wrapped inside an asynchronous function to allow for multiple consecutive inputs, I separately handle keyDown and keyUp events, where on key down the interval is added to an array of intervals (thanks to javascripts type safety), in which the interval is cleared when an OS keyUP is detected, this comes with caveats as there are operating systems which send these events at different times, which can introduce some uncertainty. But due to the timings being customizeable, this isn't much of a problem.

```js
// Example for one direction, repeated for others
case $state.keycodes.wU:
    if (dasIntervals[0] == false) {
        dasIntervals[0] = setTimeout(() => {
            dasIntervals[0] = setInterval(() => {
                wcursorY = Math.max(wcursorY - 1, 0);
                if ($state.gameMode === 'multiplayer') {
                    ws.send(JSON.stringify({
                        type: 'Move',
                        data: { player_id: `${temp_id}`, action: 'CursorBlueUp' }
                    }));
                }
                moves.push(['CursorBlueUp', Date.now() - lastActionTime]);
                lastActionTime = Date.now();
            }, $state.das);
        }, $state.dasDelay);
    }
```

=== Data Structures

==== Circular Queue
A queue is a data structure following the FIFO (first in first out) principle, where you use a sized array, along with variables to store the capacity, front & back of the array, when a file is queued, the file is put onto the index of the back of the array, and then the back index is added to % capacity unless the back becomes equal to the front, in which the queue returns an error instead, this allows for a non resizable array, which allows a set amount of elements to be queued, but not more than the size of the array, allowing for efficient memory management

==== HashMap
A hash table (colloquially called a hashmap) is an array that is abstracted over by a "hashing" function, which outputs an index based on an output, usually the hash function aims to be as diverse as possible, but you can also write special hash functions that are more efficient for your given data types.

==== Option/Result Types
an Optional type, is a simple data structure that allows for beautiful error handling, an Option type wraps the output data, allowing for the error to be handled before trying to manipulate data, i.e in a Some(data) or None, where None means that the data was nonexistent, or we can use a result type to handle errors down the stack, where we can pass the error with Err(e) and Ok(d), so if one part of the function layer breaks we can know exactly where it errored and softly handle the error if needed

==== 
=== Function Map



=== Class Diagrams


=== Database Design

#figure(
  image("assets/ERM.png", width: 80%),
  caption: [Entity Relationship Model - Database schema showing relationships between game entities]
)

The Entity Relationship Model illustrates the database structure for DoubleTapp, showing the relationships between users, games, statistics, and other critical data entities that support gameplay and analytics.

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






== Testing

#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: (left, center, center),
  stroke: 0.7pt,
  [*Test Description*], [*Status*], [*Proof*],
  
  "Test user registration with valid credentials", [Pass], [ ],
  "Test user registration with existing username", [Pass], [ ],
  "Test user login with valid credentials", [Pass], [ ],
  "Test user login with invalid credentials", [Pass], [ ],
  "Test session persistence across page reloads", [Pass], [ ],
  "Test session expiry after timeout", [Pass], [ ],
  "Test grid initialization with correct size (4x4)", [Pass], [ ],
  "Test grid initialization with correct size (5x5)", [ ], [ ],
  "Test grid initialization with correct size (6x6)", [ ], [ ],
  "Test initial cursor positions (blue at 0,0 and red at size-1,size-1)", [ ], [ ],
  "Test initial grid has exactly 'size' active tiles", [ ], [ ],
  "Test blue cursor movement in all directions with keyboard", [ ], [ ],
  "Test red cursor movement in all directions with keyboard", [ ], [ ],
  "Test cursor movement boundary limits (cannot move outside grid)", [ ], [ ],
  "Test DAS (Delayed Auto Shift) functionality for cursor movement", [ ], [ ],
  "Test valid submission when both cursors are on active tiles", [ ], [ ],
  "Test invalid submission when cursors are on the same tile", [ ], [ ],
  "Test invalid submission when one cursor is not on an active tile", [ ], [ ],
  "Test score increment on valid submission", [ ], [ ],
  "Test score reset on invalid submission", [ ], [ ],
  "Test visual feedback (green) for correct submissions", [ ], [ ],
  "Test visual feedback (red) for incorrect submissions", [ ], [ ],
  "Test new active tiles appear after valid submission", [ ], [ ],
  "Test deactivation of submitted tiles after valid submission", [ ], [ ],
  "Test timer countdown functionality", [ ], [ ],
  "Test game end when timer reaches zero", [ ], [ ],
  "Test game statistics display after game end", [ ], [ ],
  "Test leaderboard display with correct pagination", [ ], [ ],
  "Test leaderboard filtering by grid size", [ ], [ ],
  "Test leaderboard filtering by time limit", [ ], [ ],
  "Test leaderboard filtering for personal bests", [ ], [ ],
  "Test multiplayer game joining functionality", [ ], [ ],
  "Test multiplayer game quota system", [ ], [ ],
  "Test multiplayer game player elimination", [ ], [ ],
  "Test multiplayer game final rankings", [ ], [ ],
  "Test WebSocket connection establishment", [ ], [ ],
  "Test WebSocket message handling for different action types", [ ], [ ],
  "Test WebSocket reconnection on connection loss", [ ], [ ],
  "Test server-side move verification with valid moves", [ ], [ ],
  "Test server-side move verification with invalid moves", [ ], [ ],
  "Test server-side timing verification for normal play", [ ], [ ],
  "Test server-side timing verification for suspicious patterns", [ ], [ ],
  "Test server-side path optimization detection", [ ], [ ],
  "Test PRNG (Xoshiro256+) deterministic output with same seed", [ ], [ ],
  "Test game state persistence in database", [ ], [ ],
  "Test user statistics update after game completion", [ ], [ ],
  "Test keybind customization persistence", [ ], [ ],
  "Test settings reset to defaults", [ ], [ ],
  "Test game performance with rapid inputs", [ ], [ ],
  "Test game performance with simultaneous inputs", [ ], [ ]
)


#pagebreak()
= Bibliography
#bibliography(
  "bibliography.yml",
  title:none,
  full:true,
  style: "ieee"
)

=== Function & Flow Diagrams

#figure(
  image("assets/gamehandler-flowchart.png", width: 80%),
  caption: [Game Handler Flowchart - Main processing logic for handling game state transitions and events]
)

This flowchart illustrates the core game handling process, showing how user inputs are processed, game state is updated, and rendering occurs in the main game loop.

#figure(
  image("assets/gamehandler-prototype-flowchart.png", width: 80%),
  caption: [Game Handler Prototype Flowchart - Early design of the game processing pipeline]
)

The prototype flowchart shows the initial design approach for the game handler before refinements were implemented, demonstrating the evolution of the system architecture.

#figure(
  image("assets/singleplayer-game-flowchart.png", width: 80%),
  caption: [Singleplayer Game Flowchart - Logic flow for the timer-based singleplayer mode]
)

This diagram details the timer-based singleplayer game mode logic, including state initialization, score tracking, and game termination conditions.

#figure(
  image("assets/multiplayer-game-flowchart.png", width: 80%),
  caption: [Multiplayer Game Flowchart - Communication and state management for networked gameplay]
)

The multiplayer flowchart illustrates the client-server communication pattern, synchronization mechanisms, and player state management required for consistent networked gameplay.

#figure(
  image("assets/websockets.png", width: 80%),
  caption: [WebSocket Architecture - Implementation of the bidirectional communication system]
)

This diagram shows the WebSocket implementation architecture, detailing how persistent connections are established, maintained, and utilized for real-time game updates between clients and server.
