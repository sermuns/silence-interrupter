convert-audio:
	fd -I -e mp3 -x ffmpeg -loglevel error -stats -i {} {.}.ogg
