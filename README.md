# Welcome

Hello! This is a fun personal project to create an easy-to-use wrapper on the ESPN Fantasy Football API. It draws inspiration from [this NodeJS](http://espn-fantasy-football-api.s3-website.us-east-2.amazonaws.com/) based client. Much of this client is a recreation of that work, without which I'd have had practically no starting point.

Originally, I wanted to rewrite a Typescript script in Rust for practice. This set me on the journey of recreating the entire ESPN API client needed to get the initial data. Actually rewriting my script was far and away the fastest portion. Hand writing structs for the API Responses was the bulk of the work for this crate.

## Details

The primary output of this crate is the EspnClient, which is a reqwest::Client with helper methods to build requests and parse them into Strongly Typed values for easy scripting.

I'm new to Rust development. I don't own any libraries in other languages. I'm sure there are many optimizations that could be made here -- I welcome any pull requests. The idea is to make something easy to use so you, too, can script your way to extra fun with your ESPN league.

## Other projects

You can see some of my scripts written using this client library.

- [Scripts Repo](https://github.com/Styerp/fantasy-football/tree/main)
