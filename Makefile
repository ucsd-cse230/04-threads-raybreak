
turnin: 
	git commit -a -m "turnin"
	git push origin main

upstream:
	git remote add upstream https://github.com/ucsd-cse230/04-threads-raybreak.git

update:
	git pull upstream main
