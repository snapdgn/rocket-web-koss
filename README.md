# Project Title #
**Rocket-Web-Koss**
<br />A simple webserver using rocket framework in rust.

# Demo-Preview

![alt text](https://github.com/chinsaw/rocket-web-koss/blob/master/clip_mod_1.png?raw=true)

***Please take a look at a short clip of this project deployment here:-*** [https://youtu.be/BqI-l-WJ8wM]

# Table of contents

- [Project Title](#project-title)
- [Demo-Preview](#demo-preview)
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
<br /> To Switch to Nightly Enter ```rustup default nightly``` in your terminal.

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
> - The endpoint is [http://0.0.0.0:8000/static] Make sure to go to the the **/static** endpoint to render the contents.The landing page is simply a "hello world".
> - *Make Sure you type the exact commands*
> - *Make sure you're in the root directory of the project (this project is not configured to use the relative path) before running `cargo run --release`, or else page may break since it can't load the css and js which is in another folder.*

# Development
[(Back to top)](#table-of-contents)

I've tried to make this thing as minimalistic as possible with minimum creates, i used only rocket version 0.4.10, for this project.

Some neat tricks i Learned:-
> - One could use `cargo add <crate name>` to easily download and include the crate in Cargo.toml file.
  You've to download cargo add first by `cargo install <crate name>`.
> - You could explore a range of crates in [crates.io](crates.io)

<a name="docker"></a>
# Docker-Build
[(Back to top)](#table-of-contents)


<a name="learnings"></a>
# What i've learned by making this project?
[(Back to top)](#table-of-contents)
</br>
Frankly, I learned a lot, i didn't know much of the frontend.So in order to render html,css,and javascript i had to learn bootstrap and also came to know about 
template rendering engines in rust such as [tera](https://tera.netlify.app/). Although i never used that in this project, it was a valuable learing experience.
I had to struggle a lot in static file rendering in rust's rocket, as rocket is a relatively new framework and don't have much documentation.I had to hike around a lot for this.




