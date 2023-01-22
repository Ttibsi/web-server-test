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
