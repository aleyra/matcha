# matcha
## Info from 42 school
### Description
This project will introduce a more evolved tool to create your web applications: the micro-framework. We invite you to create, in the language of your choice, a dating site. Interaction between users is the heart of the project! 

### Keywords
* Web
* Micro-framework 

### Skills
* Web
* DB & Data
* Security 

### Team or Solo Project
Team project of 2 students : lburnet and amiguez

### Cursus
* for RNCP level 7
* Module Web Database

## Personnal Notes
### Technology Stack
* All : ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
* Front : ![Angular](https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white)	![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)	![HTML](https://img.shields.io/badge/HTML-239120?style=for-the-badge&logo=html5&logoColor=white)	![CSS](https://img.shields.io/badge/CSS-239120?&style=for-the-badge&logo=css3&logoColor=white)	![Node.js](https://img.shields.io/badge/Node.js-43853D?style=for-the-badge&logo=node.js&logoColor=white)
* Back : ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Nickel](https://img.shields.io/badge/Nickel-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![PostgreSQL](https://img.shields.io/badge/postgresql-4169e1?style=for-the-badge&logo=postgresql&logoColor=white)

### Useful links
* Start a new angular/ts project : https://www.ganatan.com/tutorials/demarrer-avec-angular
* Start a Nickel FrameWork : https://nickel-org.github.io
* Start a Diesel Project (rust ORM): https://diesel.rs/guides/getting-started.html

### TODO
1. mettre en place un docker compose
2. lancer un projet angular/ts pour le front end

### What should we have 
 - Login through Email
 - Match with other peoples
 - See who have liked the user
 - See who the user have liked
 - A metric about `Fame rating` (up to us to define)
 - See they profile
 - Chat with them
 - Customise the user profile - Public information
   - tags :
     - custom tags (#geek, #vegan ...)
   - Special tags:
     - gender
     - sexual preferences
   - biography
   - profile picture
   - pictures 
   - username
 - Private information:
   - user email
   - real names (first and last name)
 - Get the user GPS position (if not possible Find an other way to get it, maybe Ip adresses)
   - The user can update it
   - Can be showed or hidden


### Database Structure

<details>
<summary> User </summary>

- Username
- Email
- First name
- Last name
- Biography
- tag List
- Simped list (Other User that liked the user)
- Liked list (Other User that the user Liked)

</details>


<details>
<summary> Tags </summary>

- name
</details>


<details>
<summary> Chat </summary>

 - User 1
 - User 1
 - Message List
</details>

<details>
<summary> Messages </summary>
 
- Content
- Sending time

</details>



### Notes

1. No chatting allowed if no profile Picture

