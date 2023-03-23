# Project 3: Serverless Data Engineering Pipeline 

## Introduction
This is a rust project that give you movie recommadation based on the type you choose.
The data resource is from https://github.com/justmarkham/DAT8/blob/master/data/imdb_1000.csv the first 1000 imdb movies.
The types of movies are: [ 'Crime', 'Action', 'Drama', 'Western', 'Adventure', 'Biography',
       'Comedy', 'Animation', 'Mystery', 'Horror', 'Film-Noir', 'Sci-Fi',
       'History', 'Thriller', 'Family', 'Fantasy'] 
Then we use Shuttle.rs to deploy serverless data engineering pipelines. Shuttle.rs is a Rust-based tool that simplifies the deployment process of Rust applications to AWS Lambda. By using Shuttle.rs, you can package your Rust-based data engineering pipeline into a zip file that can be uploaded to AWS Lambda. This allows us to build a serverless data engineering pipeline with Rust and take advantage of Rust's performance and safety features. The process involves creating your Rust-based pipeline, using Shuttle.rs to package it into a zip file, creating a new AWS Lambda function, uploading the zip file to your Lambda function, adding a trigger to Lambda function, testing pipeline, and monitoring it to ensure it is working correctly.  Check shuttle.rs  's github page for more information. https://github.com/shuttle-hq/shuttle      


## Demo


## Architecture
The microservice using Actix Web provides an HTTP endpoint at "/movie" that can be used to recommend movies. When a user accesses this endpoint, the service generates a random movie for the computer move based on the movie type entered by the user.

The actix_web macro is used to define the entry point to the Actix web server, which is provided by the ShuttleActixWeb type of the shuttle_service board. The movie function is the handler for the "/movie" endpoint and returns an HTTP response with the computer's move. The actix_web macro is then used to wrap the main function that sets up the Actix Web server and registers the program with the service configuration. Overall, this microservice uses Actix Web and Rust over HTTP to provide a simple implementation of the movie recommendation.


## Installation
1. Set virtual environment: 
* `python3 -m venv env`
* `source env/bin/activate`
2. Install rust: 
* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* `source "$HOME/.cargo/env"`
3. login to Shuttle.rs via https://www.shuttle.rs/login, choose github login
4. Install shuttle:
* `cargo install cargo-shuttle`
5. start the login process
* `cargo shuttle login`
6. Authenticate
`cargo shuttle login --api-key 6jiJsI8dy6QwGTQT`
7. Initialize
`cargo shuttle init`
8. Run this command to deploy your project 🥳:
`cargo shuttle deploy`



## Running Locally
To test my app locally before deploying, use: `cargo shuttle run`




## Deploy Via Shuttle
1. Run this command to deploy your project 🥳:`cargo shuttle deploy`


2. Sometimes when you deploy your project, it will show errors that need to debug:


3. Once finsh debugging your errors, deploy it again untill it succeed:

4. Your service will immediately be available at {crate_name}.shuttleapp.rs. My example:


5. Run this code in terminal: 

`curl https://movie.shuttleapp.rs/movie?movie_type=Action`

### **Reference**
1. https://docs.rs/shuttle-service/latest/shuttle_service/
2. https://docs.shuttle.rs/examples/actix