
# Project Title #
**Koss-Rust-Webserver**

<br />A simple webserver using rocket framework in rust.<br />

# Demo-Preview #
<a name="deployment-video"></a>
![alt text](https://github.com/chinsaw/rocket-web-koss/blob/master/clip_mod_1.png?raw=true)

***Please take a look at a short clip of this project deployment here:-*** [https://youtu.be/BqI-l-WJ8wM]

# Table of contents

- [Project Title](#project-title)
- [Demo-Preview](#demo-preview)
- [Deployment-Yotube-Video-Link](#deployment-video)
- [Table of contents](#table-of-contents)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [Docker-Build](#docker)
- [What i've learned by making this project?](#learnings)

# Installation
[(Back to top)](#table-of-contents)

## Rust Installation and Setup ##
<br />Install rust by "rustup", its simple and blazing fast installation method.Enter this command in your terminal and
follow on screen instructions.
<br />

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br /><br /> If rustup is already installed in your system simply update it by 
<br />
```
rustup update
```
<br /><br /> To check if rust is properly installed in your system run
<br />
```
rustc --version
```
<br /><br />
Please refer to offical docs of rust if you are facing problems in installation.
[Rust-Installation-Guide](https://www.rust-lang.org/tools/install)

*Since this project uses rust nightly , You have to switch over to rust nightly in order for this to work*
<br />*__Note that this step is crucial for this to work__*
<br /> To Switch to Nightly Enter ```rustup default nightly``` in your terminal.(recommended)
<br /> Or you could also set nigtly-rust on directory basis by `rustup override set nightly`. I would still recommend the upper one though.

### Repo Cloning ###

After rust installaion and setup Clone this repo.
<br />
```
https://github.com/chinsaw/rocket-web-koss.git
```
<br />*A reminder to Switch to nightly again*
<br /><br />



# Usage
[(Back to top)](#table-of-contents)

Enter these commands in succesion after cloning the repo
```
cd rocket-web-koss
cargo run --release

```


**Note:**
> - The endpoint is [http://0.0.0.0:8000/static] Make sure to go to the the **/static** endpoint for the contents to be rendered.The landing page is simply a "hello world".
> - *Make Sure you type the exact commands*
> - *Make sure you're in the root directory of the project (this project is not configured to use the relative path) before running `cargo run --release`, or else page may break since it can't load the css and js which is in another folder.*

# Development
[(Back to top)](#table-of-contents)

I've tried to make this thing as minimalistic as possible with minimum creates, i used only "rocket" version 0.4.10, for this project.

Some neat tricks i Learned:-
> - One could use `cargo add <crate name>` to easily download and include the crate in Cargo.toml file.
  You've to download cargo add first by `cargo install <crate name>`.
> - You could explore a range of crates in [crates.io](crates.io)

<a name="docker"></a>
# Docker-Build
[(Back to top)](#table-of-contents)

I've included a Dokerfile in this repo out of which a docker image could be made and deployed.
<br />
* I've used the minimalistic distro i.e.*alpine* and pulled *rust's* official image on top of that.
* I then copied the current directory to the /app directory and set it to current working directory(WORKDIR),i then switched to rust's nightly and deployed the server.
* I thought of building a image and uploading it to dockerhub, but the image size was approx 4GB,so i couldn't do it because of low bandwidth.

**This is the more or less the deployment process in docker**


<a name="learnings"></a>
# What i've learned by making this project?
[(Back to top)](#table-of-contents)
</br>
Frankly, I learned a lot, i didn't know much of the frontend,
So in order to render a good html,css,and javascript i had to learn bootstrap a bit and also came to know about 
template rendering engines in rust such as [tera](https://tera.netlify.app/).<br />
Although i never used that in this project, it was a valuable learing experience.
<br />I had to struggle a lot in static file rendering in rust's rocket, as rocket is a relatively new framework and don't have much documentation.I had to hike around a lot for this.
<br /><br />
*And most importantly i learned **docker** which was a pleasant and valuable learinig experience.*



