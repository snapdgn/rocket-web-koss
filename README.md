# Project Title #
**Rocket-Web-Koss**
<br />A simple static website using rocket framework in rust

# Demo-Preview

![alt text](https://github.com/chinsaw/rocket-web-koss/blob/master/clip2.png?raw=true)

# Table of contents

- [Project Title](#project-title)
- [Demo-Preview](#demo-preview)
- [Table of contents](#table-of-contents)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)


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
<br />*A remainder to Switch to nightly again*
<br /><br />



# Usage
[(Back to top)](#table-of-contents)

Enter these commands in succesion after cloning the repo
```
cd rocket-web-koss
cargo run --release

```
**Note:**
> - *Make Sure you type the exact commands*
> - *Make sure you're in the root directory of the project (this project is not configured to use the relative path) before running `cargo run --release`, or else page may break since it can't load the css and js which is in another folder.*

# Development
[(Back to top)](#table-of-contents)

