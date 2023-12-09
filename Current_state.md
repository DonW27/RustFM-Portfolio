# Portfolio Project 

Donald Whitehead  
CS-339R Fall 2023  

**Current state of the program:**  
  - UI is in a functional state.  
  - Program opens and views the contents of a directory.  
  - Can navigate into child directories.  
  - Can go back to parent directory.  
  - Can go directly to a specified path, both at launch or within the program.  
  - Can open most files.  
  - Considerably more stable.  
  - Supports different icons for different file types such as images.  
  - Supports renaming, copying, moving and deleting files.
  - Supports creating, copying and moving directories.


**Issues**   
  - Spent entirely too long on trying to get other viewing options and failed.  
  - Slint has a number of issues taht gave me hang-ups, spend an obsurd amount of time trying to figure out issues with icons crashing only to finally discover it was an issue with Slint's cache when trying to load icons dynamically. Had to make it ugly by loading them all in all at once.  
  - Error popups! Slint again, I can't make it use more than one popup window at a time, so if a dialog is open it will completely ignore the error.  
  - Never found a decent way to do tests with this since it involves interacting with the filesystem, I am not comfortable making automated tests that can affect people's machines so I dod all my testing by hand. I fully expect to lose points for this. Mock testing may of been a solution here but I am too unfamiliar with it.  
  - I removed the idea of having a delete directory function because I don't want that can arise with that in such a concept program.  

**Post Project Analysis** 
Honestly, this proved challenging but for all the wrong reasons. I am a bit disappointed in how this turned out because it ended up being almost entirely about figuring out how to get things to work with the UI and less to do with Rust code. What of it there is ended us being highly repetitive, some I could likely condense more but much else impossible with the nature of how Slint and Rust interact. Slint is like trying to fit a square peg into a round hole, having a sophisticated macro and IDE tooling that lets you write Slint code right in Rust is pretty neat but in the end you do have to realise you are essentually bridging two different languages. Everything has to be sanatized for Slint and thrown back and forth over the wall with closures and special Slint-approved stucts. I am inexperienced with UI integration, perhaps that is just natural for most. Slint's documentation certainly did not help in clarifying things for new users, I feel allot of it assumes the developer is already familar with such tools. Furthermore it is clear to me that Slint is more tailored for embedded devices and not so much desktop applications. It is evident in that it seems to lack multi-window support, I have not found a way to have multipule popups, etc. It would be a great tool forbuilding a UI for something on a touch screen. 

For this project though, Slint was the cost of the vast amount of my time, the Rust aspect went aboslutely smooth because most of what I needed was already firmly established in standard libraries. In the future, I would suggest if someone chooses to expore a UI project have them make sure the challenge is going to be implementing their Rust inner workings and not the interface.  