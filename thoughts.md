#Thoughts

My initial impressions on each language and it's implementation of this web
server project

### Golang
* Reading from a database is relatively simple, although most of that was
copy/paste fromm an older project
* Templating seems to be unintuitive in html files
	* Although [this link](https://astaxie.gitbooks.io/build-web-application-with-golang/content/en/07.4.html)
	seems to be really useful - probably for the whole project
* CSS is treated as reading in another file and setting it inline as a
go template, which feels both elegant and odd at the same time - I think
there's a better way to do this, but this is easier
* I've not included any validation on input - This is vital in prod
* This prototype could also use a good refactor as well
* This seemed relatively simple as a start, although I've chosen to not
bother with specific design choices as I didn't understand them when discord
users suggested them to me. If I choose to use Go for the project this is a
prototype for, I will need to do more research into router/controller/repository
pattern
* The language doesn't feel like it's fighting me, and is simple to read/write.
The common complaint of the verbosity never felt like a drawback here, despite
Go's usual `if err != nil` stuff (which I partially converted into a function
which I'd make an actual pattern in an full project).
* Templates don't seem too tricky to work with, but I don't think it'll take
too long to run into the limitations of them. Despite that, I think a lot
of what they'll be used for will be pretty simple things, and this won't be
a significant issue

### Python
* This isn't the most complicated method of creating a static web server in
the world, but flask still has a learning curve - one that I didn't really work
with and just picked up the absolute basics needed for this.
* I think I'd like to work with this at some point in the future if I can find
the best way of structuring the project - especially with the flask code held
outside the main.py
* This was the easiest library to include a css file in, although the usage of
`templates/` and `static/` directories is annoying at this small scale
    * I can see my mind changing on this in an actual project with more content
    than this simple prototype, however
* CSS was picked up without any hassle here
* Templating using jinja2 appears to be powerful enough for most tasks, and is
battle-tested in repos a whole lot larger than this. There is enough documentation
out there.

### Rust
* Working with `rusqlite` library doesn't seem to be too intuitive, although
I'm putting this down to unfamiliarity as well - it seems to use a lot of
iterators to loop through results from `SELECT` queries
* Rust is harder to work with as a language, and reading docs/finding fixes for
compliated errors are either harder to find or more difficult to find what I
need.
    * This may potentially be down to my relative lack of experience with Rust
    compared to Go, as well as the higher learning curve Rust has, generally
    making this experience more difficult than the other languages in this repo
    - altohugh this was expected at the start.
* There are multiple options out there for templating languages -- Despite using
sailfish, I think Tera may have been a bettr option.
    - Not a fan of using a separate file format for the template as it'll be
    more difficult to track over a longer period of time, and the separate
    Sailfish.toml config file here.
* Actix-web is all that I'd need (instead of a large framework like Yew or
Leptos) but the docs aren't the easiest to navifate -- this may be an
ecosystem-wide issue
* The library also seemed all over the place, with it being harder to reuse
knowledge from one function in another place -- such as using the endpoint macro
for both `get` and `post`
