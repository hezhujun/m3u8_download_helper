VIDEO_NAME = ""

parse:
	cargo run -- --file data/$(VIDEO_NAME).m3u8 --base-url ""

download:
	sh -c 'cd data/$(VIDEO_NAME); aria2c --input-file=$(VIDEO_NAME).m3u8_download_list.txt'

build:
	ffmpeg -allowed_extensions ALL -protocol_whitelist "file,http,crypto,tcp,https" -i data/$(VIDEO_NAME)/$(VIDEO_NAME).m3u8 -c copy data/$(VIDEO_NAME)/$(VIDEO_NAME).mp4
