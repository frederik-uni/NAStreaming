import json
from plexapi.library import Library
from plexapi.server import PlexServer

def get_user_input():
    baseurl = input("Enter Plex server URL (default: http://127.0.0.1:32400): ").strip() or "http://127.0.0.1:32400"
    token = input("Enter Plex token: ").strip()
    return baseurl, token

def run(baseurl, token):
    plex = PlexServer(baseurl, token)
    items = []
    for section in plex.library.sections():
        remove = len(section.locations[0]) + 1
        sec: Library = plex.library.section(section.title)
        for video in sec.search(unwatched=False) + sec.search(inProgress=True):
            if video is None:
                continue
            if video.type == "show":
                for episode in video.episodes():
                    if episode.isWatched or episode.viewOffset >= 60000:
                        items.append(create_item(episode, remove))
            else:
                if video.isWatched or video.viewOffset >= 60000:
                    items.append(create_item(video, remove))
    with open('progress.json', 'w') as file:
        file.write(json.dumps(items, indent=4))
    print("Progress saved to progress.json")

def create_item(episode, remove):
    progress = episode.duration if episode.isWatched else episode.viewOffset
    return {
        "file": episode.media[0].parts[0].file[remove:],
        "ids": [guid.id.split("://") for guid in episode.guids],
        "duration": episode.duration,
        "progress": progress
    }

if __name__ == "__main__":
    baseurl, token = get_user_input()
    run(baseurl, token)
