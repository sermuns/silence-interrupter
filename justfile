convert-audio:
	fd -I -e mp3 -x ffmpeg -loglevel error -stats -i {} {.}.ogg

push:
	git push && git push --tags
