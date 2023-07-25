# chat-tui

Adds ChatGPT to your terminal. 
You must add your apenai api key to the file in logs/key.txt

Quick start
- clone the repo
- set your api key in logs/key.txt
- use cargo run in the main directory to build and run

Hot Keys
- Normal mode
  - "a" to enter edit mode and write (ask) your query
  - "s" to open the system panel and edit the ai system
  - "esc" to quit the application
- Edit mode
  - "esc" return to normal mode
- System mode
  - "esc" return to normal mode

dependicies
- async-openai
- crossterm
- ratatui
- tokio


planned features
- insert api key in program
- mouse suport
