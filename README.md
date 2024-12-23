# Messenger Server in Rust

This is a **Messenger Server** implemented in **Rust**, designed for managing real-time group chats. It allows users to communicate by sending text messages within chat groups. The server is built using the **Tokio** async runtime and **Warp** framework to handle HTTP requests.

The server supports basic functionalities such as:

- **User Authentication**: Users can log in and authenticate.(You cannot register!!)
- **Real-Time Group Messaging**: Users can send and receive messages in real-time.
- **Message History**: Previous messages in the group chat are accessible even when you log back in. (As long as the server is running)

## Features

- **User Login**: Log in to the server with your credentials.
      <br>
      <br>
      <img src="images/login.png" width="500" height="300"/>
- **Group Messaging**: Send and receive messages in real-time within a group.
      <br>
      <br>
      <img src="images/group.png" width="800" height="500"/>
- **Persistent Storage**: Messages and online users are stored persistently.(As long as the server is running)
- **WebSocket Support**: The server uses WebSockets for real-time communication.

## Installation

### Prerequisites

1. [Rust](https://www.rust-lang.org/) (and `cargo` package manager) 
2. [Visual Studio Code](https://code.visualstudio.com/download) along with some extensions(CTRL + SHIFT + X for install extensions)
      - Live Server
      - HTML CSS Support
      - rust-analyzer (id: rust-lang.rust-analyzer)
      - Even Better TOML (id: tamasfe.even-better-toml)
      - CodeLLDB (id: vadimcn.vscode-lldb)
      
### Steps to run this code:

1. **Clone the repository**:

   ```bash
   git clone https://github.com/MarianRadu29/Messenger-Rust-Server.git
   cd Messenger-Rust-Server
   ```

2. **Install the tools listed above in prerequisites**

3. **Run server in terminal(CTRL + J) with this command `cargo run`**

4. **To be able to run the client in the browser, right-click on the `index.html` file and select the `Open with Live Server` option.**
   <br>
   <br>
   ![](images/live-server.png)