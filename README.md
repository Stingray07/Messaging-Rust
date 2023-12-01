# Rust Messaging

This is a simple chat application built with Rust and Rocket. It also has account features.

## Getting Started

The project uses the [Rocket](https://rocket.rs/) web framework, [Diesel](https://diesel.rs/) for database interactions, and [Rand](https://docs.rs/rand/) for random number generation.

## Dependencies

- [Rocket](https://rocket.rs/): Web framework for Rust.
- [Diesel](https://diesel.rs/): ORM and Query Builder for Rust.
- [Rand](https://docs.rs/rand/): Random number generation for Rust.
- [PostgreSQL](https://www.postgresql.org/): Open-source relational database.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/Stingray07/Messaging-Rust

2. Navigate to the project directory
   ```bash
   cd Messaging-Rust

3. Setup the database
   ```bash
   diesel setup

4. Build and run the application
   ```bash
   cargo build --release
   ./target/release/rocket-chat-app

5. Access the application in your web browser at http://localhost:8000.


## Routes
- **GET ROUTES**
   ```bash
   /: Home Page
   /login : Redirect to Login
   /home : User home page
- **POST ROUTES**
   ```bash
   /create_account : Create a new user account
   /login : User login
   /home : User home page (post-login) 
   /message : Post a chat message

2. Create a .env file in the project root and add your Discord bot token and Canvas API key:

   ```bash 
   DISCORD_TOKEN='your_discord_bot_token'
   CANVAS_API_KEY='your_canvas_api_key'

## Authentication
### Create Account:
- Access the create_account route to create a new user account.
### Login:
- Access the login route to log in with your account credentials.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.


## License

[MIT](https://choosealicense.com/licenses/mit/)
