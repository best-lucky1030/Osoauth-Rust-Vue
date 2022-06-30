# Osoauth-Rust-Vue

This is a Rust/Vue based project. This project shows how to use [Oso](osohq.com) for Role-Based Access Control (RBAC) authorization. The article that best explains how I was able to come up with this project can be found [here](https://hannydevelop.hashnode.dev/role-based-access-control-with-oso-rust-and-vuejs-ckiy2x3mq0u7e33s16g30hp7i). I give credit to the team who developed the [Rust-Oso quick start](https://github.com/osohq/oso-rust-quickstart), the inspiration for this project was drawn from there. I also thank the wonderful oso team for letting me try out this wonderful open source product and writing an article based on my end user experience.

## Project Set Up

The root of this project consists of two folders (backend and frontend). The backend holds the codes for Rust and Oso authorization while the frontend holds the Vue.js codes. To run this project in your working environment, navigate to the frontend of the application on your terminal `cd frontend` and install dependencies with the command below:

```
npm install
```

Run Vue.js serve command:

```
npm run serve
```

Open another terminal instance and navigate to the project's backend `cd backend`. Run the start command for Rust project:

```
cargo run
```

Navigate to `http://localhost:8080/` or `http://localhost:8081/` if port `8080` is occupied. You should be greeted with this page below

<div align="center">
<img src="https://i.ibb.co/KDRzFkJ/2.png">
</div>

Input an Admin's name and a Job ID, hit the enter button. This is a work in progress and this repository will be updated from time to time. Feel free to [contact me](https://twitter.com/AlexSakai5) If you have any questions or suggestions regarding this project.
