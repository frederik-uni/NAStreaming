import requests
import json
import time

base = "http://127.0.0.1:8085/api/v1"
headers = {'accept': 'application/json'}

def init():
    response = requests.get(base+"/init", headers=headers)
    if response.json() == False:
        response = requests.put(base+"/init", json={
            "user": {
                "name": "demo",
                "password": "password",
                "birthdate": "2025-03-23T11:45:24.751222Z",
                "admin": True
            },
            "group": {
                "name": "Anime",
                "path": "/Users/frederik/movie_files/testset/",
                "prefered_display_order": [],
                "prefered_index_order": [],
            }
        },headers=headers)
        print("Database initialized successfully")
        return response.json()["access_token"]
    else:
        response = requests.post(base+"/user/sign-in", json={
            "username": "demo",
            "password": "password"
        }, headers=headers)
        return response.json()["access_token"]

def start_scan():
    response = requests.post(base+"/lib/list", json={"limit": 100, "offset": 0}, headers=headers)
    id = response.json()["scan_groups"][0]["id"]
    response = requests.post(base+"/services/list", headers=headers)
    assert response.json() == ["scan"]
    response = requests.put(base+"/services/dispatch", json={"service": "scan", "ctx": id}, headers=headers)
    print(response, response.text)
    time.sleep(10)

def search():
    response = requests.get(base+"/metadata-provider/providers", headers=headers)
    print(json.dumps(response.json(), indent=4))
    response = requests.post(base+"/metadata-provider/search", json={
        "id": "tv-db",
        "query": "One Piece",
        "year": 1999,
        "series": True

    }, headers=headers)
    print(json.dumps(response.json(), indent=4))

def process():
    response = requests.get(base+"/file/overview-unlinked", headers=headers)
    for key, value in response.json()["data"].items():
        response = requests.post(base+"/file/list-unlinked", json={"ids": value},headers=headers)
        json = response.json();
        items_json = json["items"]
        name = items_json[0]["info"]["name"][0]
        year = next(iter((items_json[0]["info"]["year"])), None)
        scan_group_id =items_json[0]["scan_group_id"]
        response = requests.post(base+"/metadata-provider/search", json={
            "id": "tv-db",
            "query": name,
            "year": year,
            "series": True
        }, headers=headers)

        response = requests.put(base+"/entry/add", json={"scan_group_id": scan_group_id,
        "ids": ["tv-db/" + response.json()["items"][0]["id"]],
        "series": True
        }, headers=headers)
        entry_id = response.json()
        items = [{"file_id": v["file_id"], "season": v["info"]["season"][0], "episode": v["info"]["episode"][0]} for v in items_json];
        response = requests.put(base+"/file/link-entry", json={ "entry_id": entry_id,
        "items": items}, headers=headers)

if __name__ == "__main__":
    token = init()
    headers['Authorization'] =  f'Bearer {token}'
    #search()
    #start_scan()
    process()
