# html-from-txt
static site generator with txt source

generates for every txt in all subdirectories files an html file.  
the generated html is a copy of base.html with the txt copied into it.

---
documentation:
the program requires a base.html in the local directory.
base.html can have these strings in the file: 
- ```<p>no page source</p>```
  is replaced with the content of the txt
- ```no page title```
  is replaced with the txt file name

the program recursively searches .txt files from the local directory.
