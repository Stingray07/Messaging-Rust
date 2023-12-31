const username = document.getElementById("username");
const password = document.getElementById("password");
const login_button = document.getElementById("login");
const signUp_button = document.getElementById("sign");
const join_room_button = document.getElementById("room");
const room_username = document.getElementById("username_for_room");
const back_to_login = document.getElementById("back_to_login");
const first_name = document.getElementById("first_name");
const last_name = document.getElementById("last_name");
const logout_button = document.getElementById("logout");
const LOGIN_ROUTE = "http://localhost:8000/login.html";
const CREATE_ACCOUNT_ROUTE = "http://localhost:8000/create_account.html";
const HOME_ROUTE = "http://localhost:8000/home.html";
const MESSAGE_ROUTE = "http://localhost:8000/message";
const FILL_IN_FIELDS_MESSAGE = "PLEASE FILL IN ALL THE FIELDS";
const ACCOUNT_CREATION_SUCCESS_MESSAGE = "ACCOUNT CREATION SUCCESSFUL";

// ================================FUNCTIONS===============================

const handle_ok_response = () => {
  const h1elemnt = document.createElement("h1");
  h1elemnt.textContent = ACCOUNT_CREATION_SUCCESS_MESSAGE;
  document.appendChild(h1elemnt);
};

function post_request(endpoint, data) {
  var myHeaders = new Headers();
  myHeaders.append("Content-Type", "application/json");

  var raw = JSON.stringify(data);

  var requestOptions = {
    method: "POST",
    headers: myHeaders,
    body: raw,
    redirect: "follow",
  };

  fetch(endpoint, requestOptions)
    .then((response) => {
      if (response.redirected) {
        window.location.href = response.url;
      } else {
        return response.text();
      }
    })
    .then((result) => {
      if (result) {
        console.log(result);
      }
    })
    .catch((error) => console.log("error", error));
}

function getCookie(name) {
  const cookies = document.cookie.split(";");
  for (const cookie of cookies) {
    const [cookieName, cookieValue] = cookie.trim().split("=");
    if (cookieName === name) {
      return decodeURIComponent(cookieValue);
    }
  }
  return null; // Return null if the cookie with the specified name is not found
}

// ===========================LOGIN ACCOUNT JS=====================================

if (window.location.href == LOGIN_ROUTE) {
  login_button.addEventListener("click", () => {
    if (username.value === "" || password.value === "") {
      alert(FILL_IN_FIELDS_MESSAGE);
    } else {
      data = {
        username: username.value,
        password: password.value,
      };

      post_request(LOGIN_ROUTE, data);
    }
  });

  signUp_button.addEventListener("click", () => {
    window.location.href = CREATE_ACCOUNT_ROUTE;
  });
}

// ===========================CREATE ACCOUNT JS=====================================

if (window.location.href == CREATE_ACCOUNT_ROUTE) {
  signUp_button.addEventListener("click", () => {
    if (
      username.value === "" ||
      password.value === "" ||
      first_name.value === "" ||
      last_name.value === ""
    ) {
      alert(FILL_IN_FIELDS_MESSAGE);
    } else {
      data = {
        username: username.value,
        password: password.value,
        first_name: first_name.value,
        last_name: last_name.value,
      };

      post_request(CREATE_ACCOUNT_ROUTE, data);
    }
  });

  back_to_login.addEventListener("click", () => {
    window.location.href = LOGIN_ROUTE;
  });
}

// ====================================== HOME ROUTE =====================================

if (window.location.href == HOME_ROUTE) {
  console.log(document.cookie);
  logout_button.addEventListener("click", () => {
    console.log(document.cookie);
    let data = {
      status: "logout",
    };

    post_request(HOME_ROUTE, data);
  });

  const eventSource = new EventSource("/events");
  const messages = document.getElementById("messages");
  const input = document.getElementById("input");
  const button = document.getElementById("send_button");

  eventSource.onmessage = (event) => {
    const data = JSON.parse(event.data);
    const messageElement = document.createElement("li");
    messageElement.textContent = data.username + ": " + data.message;
    messages.appendChild(messageElement);
  };

  //get cookies
  button.addEventListener("click", () => {
    const username = getCookie("username");
    console.log(username);
    const message = input.value;
    const data = {
      username: username,
      message: message,
    };
    if (message.trim() !== "") {
      input.value = "";
      post_request(MESSAGE_ROUTE, data);
    }
  });
}
