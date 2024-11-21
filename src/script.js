const ws = new WebSocket("ws://127.0.0.1:8081");

const loginSection = document.getElementById("auth-section");
const chatSection = document.getElementById("chat-section");
const messagesDiv = document.getElementById("messages");
const statusUsersDiv = document.getElementById('status-users');
let currentUser = '';
let userList = [];

// Deschiderea conexiunii WebSocket
ws.onopen = () => {
    console.log("Conexiunea WebSocket a fost deschisa.");
};

ws.onerror = (error) => {
    console.error("Eroare WebSocket:", error);
};

ws.onclose = () => {
    console.log("Conexiunea WebSocket a fost inchisa.");
};

// Handler pentru login (cand apasa Enter in campul de parola)
document.getElementById('password').addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
        document.getElementById("login-button").click();
    }
});

// Handler pentru butonul de login
document.getElementById("login-button").onclick = () => {
    const username = document.getElementById("username").value;
    const password = document.getElementById("password").value;

    if (username && password) {
        const loginMessage = `login:${username}:${password}`;
        ws.send(loginMessage);
        console.log("Mesaj de autentificare trimis:", loginMessage);
    } else {
        alert("Introduceti un username si o parola!");
    }
};

// Primesc mesajele de la server
ws.onmessage = (event) => {
    const message = event.data;

    let data = JSON.parse(message); // Presupunem ca mesajul primit este un JSON

    switch (data.type_msg) {
        case 1: {
            alert("Autentificare esuata! Incercati din nou.");
            document.getElementById("username").value = '';
            document.getElementById("password").value = '';
            break;
        }
        case 2: {
            //alert(`${data.content}`);
            userList = JSON.parse(data.content);
            currentUser = userList[0];
            loginSection.style.display = "none";
            chatSection.style.display = "flex";
            break;
        }
        case 3: {
            alert(`Utilizatorul este deja logat in chat!`);
            document.getElementById("username").value = '';
            document.getElementById("password").value = '';
            break;
        }
        case 4: {
            let db = JSON.parse(data.content);
            db.forEach(obj => {
                const messageWrapper = document.createElement("div"); // Wrapper pentru mesaj si nume
                messageWrapper.className = "message-wrapper";

                const senderElement = document.createElement("div"); // Numele utilizatorului
                senderElement.textContent = obj.sender;
                senderElement.className = "message-sender";

                const messageElement = document.createElement("div"); // Continutul mesajului
                messageElement.textContent = obj.content;
                messageElement.className = obj.sender === currentUser ? "message from-client" : "message from-server";
                senderElement.style.textAlign = obj.sender == currentUser? "end" : "left";
                messageWrapper.appendChild(senderElement);
                messageWrapper.appendChild(messageElement);

                messagesDiv.appendChild(messageWrapper);
            });
            messagesDiv.scrollTop = messagesDiv.scrollHeight; // Scroll automat catre cel mai recent mesaj
            break;
        }
        case 5: {
            
            let data_content = JSON.parse(data.content);
            if(data_content.sender!=currentUser){
                const messageWrapper = document.createElement("div"); // Wrapper pentru mesaj si nume
                messageWrapper.className = "message-wrapper";
    
                const senderElement = document.createElement("div"); // Numele utilizatorului
                senderElement.textContent = data_content.sender;
                senderElement.className = "message-sender";
    
                const messageElement = document.createElement("div"); // Continutul mesajului
                messageElement.textContent = data_content.content;
                messageElement.className = "message from-server";
    
                messageWrapper.appendChild(senderElement);
                messageWrapper.appendChild(messageElement);
    
                messagesDiv.appendChild(messageWrapper);
                messagesDiv.scrollTop = messagesDiv.scrollHeight; // Scroll automat catre cel mai recent mesaj
            }
            
            break;
        }
        case 6:
            {
                let data_content = JSON.parse(data.content);
                //alert(`${JSON.stringify(data_content)}`);
                updateUserStatus(userList,data_content);
                break;
            }
    }
};

// Handler pentru apasarea Enter la trimiterea unui mesaj
document.getElementById("message").addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
        document.getElementById("send-button").click();
    }
});

// Handler pentru butonul de trimitere a mesajului
document.getElementById("send-button").onclick = () => {
    const message = document.getElementById("message").value;
    if (message.trim()) {
        let s = {
            sender: currentUser,
            content: message
        };
        ws.send(JSON.stringify(s)); // Trimite mesajul catre server
        console.log("Mesaj trimis:", message);

        const messageWrapper = document.createElement("div"); // Wrapper pentru mesaj si nume
        messageWrapper.className = "message-wrapper";

        const senderElement = document.createElement("div"); // Numele utilizatorului
        senderElement.textContent = currentUser;
        senderElement.className = "message-sender";
        senderElement.style.textAlign = "end";

        const messageElement = document.createElement("div"); // Continutul mesajului
        messageElement.textContent = message;
        messageElement.className = "message from-client";

        messageWrapper.appendChild(senderElement);
        messageWrapper.appendChild(messageElement);

        messagesDiv.appendChild(messageWrapper); // Adauga mesajul in sectiunea de mesaje
        messagesDiv.scrollTop = messagesDiv.scrollHeight; // Scroll automat catre cel mai recent mesaj

        document.getElementById("message").value = ""; // Clear input
    }
};

// Detecteaza inchiderea sau schimbarea paginii
window.addEventListener('beforeunload', function (event) {
    ws.send("close");
});

// Detecteaza inchiderea efectiva a paginii
window.addEventListener('unload', function () {
    ws.send("close");
});
function updateUserStatus(allUser,connectedUsers) {
    // Golim lista de utilizatori
    statusUsersDiv.innerHTML = '';

    // Adaugam un status pentru fiecare utilizator
    allUser.forEach(user => {
        const userDiv = document.createElement('div');
        userDiv.classList.add('user-status');

        const statusCircle = document.createElement('div');
        statusCircle.classList.add('status-circle');
        statusCircle.classList.add(connectedUsers.includes(user) ? 'connected' : 'offline');

        const userName = document.createElement('p');
        userName.textContent = user;

        userDiv.appendChild(statusCircle);
        userDiv.appendChild(userName);
        statusUsersDiv.appendChild(userDiv);
    });
}

setInterval(()=>{
    if(currentUser!=''){
        ws.send("status");
    }
},10)