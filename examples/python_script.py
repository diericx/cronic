#!/usr/bin/python3
import subprocess
import requests

backups = [
    ('backup_1', 'restic --help'),
    ('backup_2', 'restic --help')
]

for (name, command) in backups:
    exit_code, output = subprocess.getstatusoutput(command)
    requests.post('http://0.0.0.0:8000/events/new', json={
        'source': name,
        'code': exit_code,
        'output': output
    })
