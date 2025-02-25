#STEPS TO ACHIEVE THE PROJECT.
# fibbot_Day1

This example of a GitHub Action in Rust.

##DAYS 1:

### Milestone 1: How to create a custom GitHub Actions in Rust.

#####First possibility:

- [] Install the tool `cargo generate`: 
```sh
cargo-binstall cargo generate 
 
```
 
- [] Create a new template for the action:
-  
```sh
cargo generate dbanty/rust-github-action-template
```


#####Second possibility:

- [] Use a github repository template: 
eg. <!-- https://github.com/dbanty/sample-rust-action -->

### Milestone2: new cargo project + action.yml file
- [] New cargo project: `cargo new fibbot`
- [] action.yml : `touch action.yml`. Copy the action.yml's content from the template to you own action.yml

#End-of-Day Outcome: 
- [] Create a repository with a template GitHub action and understand each action.
