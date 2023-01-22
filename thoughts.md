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
