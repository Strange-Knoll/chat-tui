# chat-tui
ChatGPT right in your terminal.

Quick start
- clone the repo
- use cargo run in the main directory to build and run
- paste your api key in the app. press (k) to enter api key mode and (enter) to paste your key

Hot Keys
- Normal mode
  - "a" to enter edit mode and write (ask) your query
  - "s" to open the system panel and edit the ai system
  - "k" to open the api key panel
  - "esc" to quit the application
- Edit mode
  - "esc" return to normal mode
- System mode
  - "esc" return to normal mode
- Api Key mode
  - "enter" paste api-key from clipboard
  - "esc" return to normal mode

dependicies
- async-openai
- crossterm
- ratatui
- tokio
- cli-clipboard


planned features
- mouse suport
