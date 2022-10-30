OUTPUT=$(restic --help 2>&1)
CODE=$?

curl -X POST \
  -d "source=restic_backup_home_zac" \
  -d "code=$CODE" \
  --data-urlencode "output=$OUTPUT" \
  http://0.0.0.0:8000/events/new
