import requests
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
                "path": "/Users/frederik/movie_files",
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
    response = requests.post(base+"/services/list", json={"limit": 100, "offset": 0}, headers=headers)
    assert response.json() == ["scan"]
    response = requests.put(base+"/services/dispatch", json={"service": "scan", "ctx": id}, headers=headers)
    print(response, response.text)

if __name__ == "__main__":
    token = init()
    headers['Authorization'] =  f'Bearer {token}'
    start_scan()
